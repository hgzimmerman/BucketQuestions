use crate::{
    error::{
        DependentConnectionError,
        Error::{self},
    },
    get_google_login_link,
    state::{HttpsClient, State},
};
use askama::Template;
use authorization::{JwtPayload, Secret};
use db::user::db_types::{NewUser, User};
use futures::{future::Future, stream::Stream};
use hyper::{body::Body, Chunk, Request, Response};
use log::{error, info, warn};
use oauth2::{
    basic::{BasicClient},
};
use serde::{Deserialize, Serialize};
use url::Url;
use warp::{filters::BoxedFilter, path, query::query, Filter, Reply};
use db::BoxedRepository;

/// The path segment for the auth api.
pub const AUTH_PATH: &str = "auth";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkResponse {
    link: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OAuthRedirectQueryParams {
    code: String,
    state: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
    id_token: String,
}

#[derive(Clone, Debug, Deserialize)]
struct GoogleJWTPayload {
    sub: String,
    name: Option<String>,
}

/// The login flow is as follows:
/// * User gets the link from /api/auth/link
/// * User clicks link
/// * User is taken to Google login portal
/// * User selects account
/// * User is redirected to /api/auth/redirect
/// * Code is extracted from query params and used to send request to google for a google identity JWT
/// * JWT is decoded and the user ID is extracted.
/// * The id is used to look up or create a new user.
/// * The new user is serialized as part of a new JWT.
/// * The JWT is templated into a small html page, that executes a script to put the JWT in localStorage.
/// * The page then redirects to a known page.
pub fn auth_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    //impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    let get_link = path!("link")
        .and(warp::get2())
        .and(state.google_client())
        .map(|google_client: BasicClient| {
            //            let redirect_url = url::Url::parse("http://localhost:8080/api/auth/redirect").unwrap();
            let link = get_google_login_link(google_client);
            info!("Generating link: {}", link);
            LinkResponse {
                link: link.to_string(),
            }
        })
        .map(crate::util::json);

    // TODO validate CSRF token
    // This means that the CSRF token should be uniform across multiple requests?

    let redirect_url = state.redirect_url();

    let redirect = path!("redirect")
        .and(warp::get2())
        .and(query())
        .map(|query_params: OAuthRedirectQueryParams| query_params.code)
        .map(move |token| create_token_request(token, redirect_url.clone()))
        .and_then(crate::util::reject)
        .and(state.https_client())
        .and_then(|request, client| {
            make_request_for_google_jwt_token(request, client).map_err(Error::reject)
        })
        .map(
            |response: TokenResponse| -> Result<GoogleJWTPayload, Error> {
                extract_payload_from_google_jwt(&response.id_token)
            },
        )
        .and_then(crate::util::reject)
        .and(state.db2())
        .map(get_or_create_user)
        .and_then(crate::util::reject)
        .and(state.secret())
        .map(create_jwt)
        .and_then(crate::util::reject)
        .map(|jwt: String| login_template_render(&jwt, "/"))
        .with(warp::reply::with::header("content-type", "text/html"));

    path(AUTH_PATH).and(get_link.or(redirect)).boxed()
}

/// The url for getting Google's JWT
const GOOGLE_JWT_URL: &str = "https://www.googleapis.com/oauth2/v4/token";

/// Creates the request used in getting the JWT from Google.
fn create_token_request(token: String, redirect_url: Url) -> Result<Request<Body>, Error> {
    // TODO get these from some central state.
    let google_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .expect("Missing the GOOGLE_CLIENT_SECRET environment variable.");
    let google_id = std::env::var("GOOGLE_CLIENT_ID")
        .expect("Missing the GOOGLE_CLIENT_ID environment variable.");
    // TODO get this from some central state.
    let redirect_uri = &redirect_url.to_string();

    let code = token;
    #[derive(Serialize)]
    struct OAuthTokenRequest<'a> {
        code: String,
        client_id: String,
        client_secret: String,
        redirect_uri: &'a str,
        grant_type: &'a str,
    }

    let body = OAuthTokenRequest {
        code,
        client_id: google_id,
        client_secret: google_secret,
        redirect_uri,
        grant_type: "authorization_code",
    };

    let body = serde_urlencoded::to_string(body).map_err(|e| {
        error!("{}", e);
        Error::DependentConnectionFailed(DependentConnectionError::Context(
            "Could not format body for dependent request for google oauth".to_string(),
        ))
    })?;

    info!("{}", body);

    Request::post(GOOGLE_JWT_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(body))
        .map_err(|_| {
            Error::DependentConnectionFailed(DependentConnectionError::Context(
                "Could not create body for dependent request to google oauth".to_string(),
            ))
        })
}

/// Make the request to google for a JWT token.
fn make_request_for_google_jwt_token(
    request: Request<Body>,
    https_client: HttpsClient,
) -> impl Future<Item = TokenResponse, Error = Error> {
    https_client
        .request(request)
        .map_err(|e| {
            warn!("requesting token failed: {}", e);
            Error::DependentConnectionFailed(DependentConnectionError::UrlAndContext(
                GOOGLE_JWT_URL.to_string(),
                e.to_string(),
            ))
        })
        .and_then(|response: Response<Body>| {
            response
                .into_body()
                .concat2()
                .map_err(|_| Error::internal_server_error("Could not deserialize body")) // Await the whole body
        })
        .and_then(|chunk: Chunk| {
            let v = chunk.to_vec();
            let body = String::from_utf8_lossy(&v).to_string();

            let response: TokenResponse = serde_json::from_str(&body).map_err(|_| {
                Error::InternalServerError(Some(format!("Could not parse response {}", body)))
            })?;
            Ok(response)
        })
}

/// Extracts the payload from the JWT provided by Google.
fn extract_payload_from_google_jwt(jwt: &str) -> Result<GoogleJWTPayload, Error> {
    let payload = jwt
        .split(".")
        .nth(1) // get the second part
        .ok_or_else(|| Error::internal_server_error("Google JWT was malformed"))?;
    let payload = base64::decode(payload)
        .map_err(|_| Error::internal_server_error("Google JWT payload decode failure"))?;
    let payload_string = String::from_utf8_lossy(&payload).into_owned();
    info!("{}", payload_string);

    serde_json::from_slice::<GoogleJWTPayload>(&payload)
        .map_err(|_| Error::internal_server_error("Google JWT could not be deserialized"))
}

/// Gets or creates a user.
fn get_or_create_user(
    google_jwt_payload: GoogleJWTPayload,
    conn: BoxedRepository,
) -> Result<User, Error> {
    use diesel::result::Error as DieselError;
    conn.get_user_by_google_id(google_jwt_payload.sub.clone())
        .or_else(|error| {
            if let DieselError::NotFound = error {
                let new_user = NewUser {
                    google_user_id: google_jwt_payload.sub,
                    google_name: google_jwt_payload.name,
                };
                conn.create_user(new_user)
                    .map_err(|_| Error::DatabaseError("Could not create user".to_string()))
            } else {
                Err(Error::DatabaseError(
                    "Could not get User. User may exist, but something else went wrong in the database".to_owned(),
                ))
            }
        })
}

/// Creates the jwt from a User record.
fn create_jwt(user: User, secret: Secret) -> Result<String, Error> {
    let lifetime = chrono::Duration::weeks(30);
    let payload: JwtPayload<User> = JwtPayload::new(user, lifetime);
    payload.encode_jwt_string(&secret).map_err(Error::from)
}

/// Login by sending a small html page that inserts the JWT into localstorage
/// and then redirects to the main page.
///
/// # Note
/// The JWT is stored in window.localstorage under the key: 'jwt'
fn login_template_render(jwt: &str, target_url: &str) -> String {
    #[derive(Template)]
    #[template(path = "login.html")]
    struct LoginTemplate<'a> {
        jwt: &'a str,
        target_url: &'a str,
    }
    let login = LoginTemplate { jwt, target_url };
    login.render().unwrap_or_else(|e| e.to_string())
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::state::test_util::execute_test_on_repository;
    use db::test::empty_fixture::EmptyFixture;
    use db::test::user_fixture::UserFixture;
    use db::RepositoryProvider;
    use db::test::user_fixture::{TEST_GOOGLE_NAME, TEST_GOOGLE_USER_ID};

    /// Gets a basic JWT from the state for use in testing.
    ///
    /// It uses the same "credentials" as used by the fixtures created in the `db` library.
    ///
    /// # Panics
    /// Panics if the database call to create a user fails.
    /// Or if the JWT can't be encoded.
    ///
    #[cfg(test)]
    pub fn get_jwt(state: &State) -> String {
        let secret: Secret = warp::test::request().filter(&state.secret()).unwrap();
        let conn: BoxedRepository = warp::test::request().filter(&state.db2()).unwrap();

        let google_jwt_payload = GoogleJWTPayload {
            sub: TEST_GOOGLE_USER_ID.to_string(),
            name: Some(TEST_GOOGLE_NAME.to_string())
        };
        let user = get_or_create_user( google_jwt_payload, conn).expect("Should get or create user.");
        create_jwt(user, secret).expect("Should create JWT.")
    }

    /// Test for testing infrastructure
    #[test]
    fn get_jwt_util_creates_user() {
        execute_test_on_repository(|_fix: &EmptyFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider.clone(), Secret::new("hello"));

            let repo = provider.get_repo().expect("Should get repo.");
            repo.get_user_by_google_id(TEST_GOOGLE_USER_ID.to_string()).expect_err("User should not exist");

            let _jwt = get_jwt(&state);

            repo.get_user_by_google_id(TEST_GOOGLE_USER_ID.to_string()).expect("User should now exist");
        });
    }

    /// Test for testing infrastructure
    #[test]
    fn get_jwt_util_gets_user() {
        execute_test_on_repository(|_fix: &UserFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider.clone(), Secret::new("hello"));

            let repo = provider.get_repo().expect("Should get repo.");
            repo.get_user_by_google_id(TEST_GOOGLE_USER_ID.to_string()).expect("User should already exist");

            let _jwt = get_jwt(&state);
        });
    }
}
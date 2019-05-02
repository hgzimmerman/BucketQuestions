use crate::state::{State, HttpsClient};
use warp::{Filter, Reply, Rejection};
use warp::path;
use crate::get_google_login_link;
use log::info;
use log::warn;
use log::error;
use serde::{Serialize, Deserialize};
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::{AuthorizationCode, StandardTokenResponse, TokenType, EmptyExtraTokenFields};
use oauth2::prelude::SecretNewType;
use warp::query::query;
use crate::error::{Error, DependentConnectionError};
use hyper::{Request, Response, Chunk};
use hyper::body::Body;
use crate::error::Error::AuthError;
use futures::future::Future;
use futures::stream::Stream;
use db::user::{User, UserRepository, NewUser};
use pool::PooledConn;
use authorization::{JwtPayload, Secret};
use askama::Template;

pub const AUTH_PATH: &str = "auth";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkResponse {
    link: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OAuthRedirectQueryParams {
    code: String,
    state: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
    id_token: String,
}


pub fn auth_api(state: &State) -> impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    let get_link = path!("link")
        .and(warp::get2())
        .and(state.google_client())
        .map(| google_client: BasicClient| {
            let redirect_url = url::Url::parse("http://localhost:8080/api/auth/redirect").unwrap();
            let link = get_google_login_link(google_client);
            info!("Generating link: {}", link);
            LinkResponse { link: link.to_string() }
        })
        .map(crate::util::json);

    // TODO validate CSRF token
    // This means that the CSRF token should be uniform across multiple requests?
    let redirect = path!("redirect")
        .and(warp::get2())
        .and(query())
//        .and(state.google_client())
        .map(|query_params: OAuthRedirectQueryParams| {
            query_params.code
        })
        .and_then(|token|create_token_request(token).map_err(Error::reject))
        .and(state.https_client())
        .and_then(make_request_for_google_jwt_token)
        .map(|response: TokenResponse| -> Result<String, Error> {
            extract_user_id_from_google_jwt(&response.id_token)
        })
        .and_then(crate::util::reject)
        .and(state.db())
        .map(get_or_create_user)
        .and_then(crate::util::reject)
        .and(state.secret())
        .map(|user: User, secret: Secret| {
            let lifetime = chrono::Duration::weeks(30);
            let payload = JwtPayload::new(user, lifetime);
            payload.encode_jwt_string(&secret)
                .map_err(Error::from)
        })
        .and_then(crate::util::reject)
        .map(|jwt: String| {
            login_template_render(&jwt, "/")
        })
        .with(warp::reply::with::header("content-type", "text/html"));


    path(AUTH_PATH)
        .and(get_link
            .or(redirect)
        )
}

const URL: &str = "https://www.googleapis.com/oauth2/v4/token";
fn create_token_request(token: String /*StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>*/) -> Result<Request<Body>,Error> {
    // TODO get these from some central state.
    let google_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .expect("Missing the GOOGLE_CLIENT_SECRET environment variable.");
    let google_id = std::env::var("GOOGLE_CLIENT_ID")
        .expect("Missing the GOOGLE_CLIENT_ID environment variable.");
    // TODO get this from some central state.
    let redirect_uri = "http://localhost:8080/api/auth/redirect";


    let code = token;
    #[derive(Serialize)]
    struct OAuthTokenRequest<'a> {
        code: String,
        client_id: String,
        client_secret: String,
        redirect_uri: &'a str,
        grant_type: &'a str
    }

    let body = OAuthTokenRequest {
        code,
        client_id: google_id,
        client_secret: google_secret,
        redirect_uri,
        grant_type: "authorization_code"
    };

    let body = serde_urlencoded::to_string(body)
        .map_err(|e| {
            error!("{}", e);
            Error::DependentConnectionFailed(DependentConnectionError::Context("Could not format body for dependent request for google oauth".to_string()))
        })?;

    info!("{}", body);


    Request::post(URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(body))
        .map_err(|_| {
            Error::DependentConnectionFailed(DependentConnectionError::Context("Could not create body for dependent request to google oauth".to_string()))
        })
}


fn make_request_for_google_jwt_token(request: Request<Body>, https_client: HttpsClient) -> impl Future<Item = TokenResponse, Error = Rejection> {
    https_client.request(request)
        .map_err(|e|{
            warn!("requesting token failed: {}", e);
            Error::DependentConnectionFailed(DependentConnectionError::UrlAndContext(URL.to_string(), e.to_string())).reject()
        })
        .and_then(|response: Response<Body>| {
            response.into_body().concat2()
                .map_err(|_|Error::internal_server_error("Could not deserialize body").reject()) // Await the whole body
        })
        .and_then(|chunk: Chunk| {
            let v = chunk.to_vec();
            let body = String::from_utf8_lossy(&v).to_string();

            let response: TokenResponse = serde_json::from_str(&body).map_err(|_| {
                Error::InternalServerError(Some(format!("Could not parse response {}", body))).reject()
            })?;
            Ok(response)
        })
}

fn extract_user_id_from_google_jwt(jwt: &str) -> Result<String, Error> {
    let payload = jwt.split(".")
        .nth(1) // get the second part
        .ok_or_else(|| Error::internal_server_error("Google JWT was malformed"))?;
    let payload = base64::decode(payload)
        .map_err(|_| Error::internal_server_error("Google JWT payload decode failure"))?;
    let payload_string = String::from_utf8_lossy(&payload).into_owned();
    info!("{}", payload_string);
    #[derive(Debug, Deserialize)]
    struct Payload {
        sub: String
    }
    serde_json::from_slice::<Payload>(&payload)
        .map_err(|_| Error::internal_server_error("Google JWT could not be deserialized"))
        .map(|payload| payload.sub)
}

fn get_or_create_user(id: String, conn: PooledConn) -> Result<User, Error> {
    use diesel::result::Error as DieselError;
    conn.get_user_by_google_id(id.clone())
        .or_else(|error| {
            if let DieselError::NotFound = error {
                let new_user = NewUser {
                    google_user_id: id
                };
                conn.create_user(new_user).map_err(|_| Error::DatabaseError("Could not create user".to_string()))
            } else {
                Err(Error::DatabaseError("Could not get User. User may exist, but something else went wrong".to_owned()))
            }
        })
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
    let login = LoginTemplate {
        jwt,
        target_url,
    };
    login.render().unwrap_or_else(|e| e.to_string())
}


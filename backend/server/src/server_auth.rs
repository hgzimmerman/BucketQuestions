//! Provides utilities for dealing with authentication constructs in warp.
//!
//! This module exists in the server crate and not in the dedicated `auth` crate because
//! warp's semantics require unification over errors.
//! In order to implement these fallible filters, they had to have access to the error type,
//! which can only be done in the server crate, assuming that errors are not migrated to their own crate,
//! which is a situation that should be avoidable.
//!
//!

use crate::{error::Error, state::State};
use authorization::{JwtPayload, Secret, AUTHORIZATION_HEADER_KEY};
use db::user::db_types::User;
use oauth2::{
    basic::BasicClient, prelude::*, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Rejection};

/// Creates the client used to contact Google for OAuth.
pub fn create_google_oauth_client(redirect_url: Url) -> BasicClient {
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    );
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new(
        Url::parse("https://accounts.google.com/o/oauth2/v2/auth")
            .expect("Invalid authorization endpoint URL"),
    );
    let token_url = TokenUrl::new(
        Url::parse("https://www.googleapis.com/oauth2/v3/token")
            .expect("Invalid token endpoint URL"),
    );

    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .add_scope(Scope::new(
        "https://www.googleapis.com/auth/plus.me".to_string(),
    ))
    .add_scope(Scope::new("profile".to_string()))
    .set_redirect_url(RedirectUrl::new(redirect_url));
    client
}

/// Gets the login link for Google OAuth.
pub fn get_google_login_link(client: BasicClient) -> Url {
    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = client.authorize_url(CsrfToken::new_random);
    authorize_url
}

/// This filter will attempt to extract the JWT bearer token from the header Authorization field.
/// It will then attempt to transform the JWT into a usable JwtPayload that can be used by the app.
///
pub(crate) fn jwt_filter<T>(s: &State) -> BoxedFilter<(JwtPayload<T>,)>
where
    for<'de> T: Serialize + Deserialize<'de> + Send,
{
    warp::header::header::<String>(AUTHORIZATION_HEADER_KEY)
        .or_else(|_: Rejection| Error::not_authorized("Token Required").reject_result())
        .and(s.secret())
        .and_then(|bearer_string: String, secret: Secret| {
            JwtPayload::extract_jwt(bearer_string, &secret)
                .and_then(JwtPayload::validate_dates)
                .map_err(warp::reject::custom)
        })
        .boxed()
}

/// If the user has a JWT, then the user has basic user privileges.
///
/// # Arguments
/// * s - The state used to validate the JWT
pub fn user_filter(s: &State) -> BoxedFilter<(Uuid,)> {
    warp::any()
        .and(jwt_filter(s))
        .map(JwtPayload::subject)
        .map(|subject: User| -> Uuid { subject.uuid })
        .boxed()
}

/// Gets an Option<UserUuid> from the request.
/// Returns Some(user_uuid) if the user has a valid JWT, and None otherwise.
///
/// # Arguments
/// * s - The state used to validate the JWT.
pub fn optional_user_filter(s: &State) -> BoxedFilter<(Option<Uuid>,)> {
    user_filter(s)
        .map(Some)
        .or(warp::any().map(|| None))
        .unify::<(Option<Uuid>,)>()
        .boxed()
}

#[cfg(test)]
mod unit {
    use super::*;
    use crate::config::RepositoryType;
    use crate::state::state_config::{RunningEnvironment, StateConfig};
    use authorization::BEARER;
    use chrono::Duration;

    #[test]
    fn pass_jwt_filter() {
        let secret = Secret::new_hmac("yeet".to_string());
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None,
            server_lib_root: None,
            environment: RunningEnvironment::default(),
            repository: RepositoryType::Fake,
        };
        let state = State::new(conf);
        let uuid = Uuid::new_v4();
        let jwt = JwtPayload::new(uuid, Duration::weeks(2));
        let jwt = jwt.encode_jwt_string(&secret).unwrap();

        let filter = jwt_filter::<Uuid>(&state);

        assert!(warp::test::request()
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .matches(&filter))
    }

    #[test]
    fn does_not_pass_jwt_filter() {
        let secret = Secret::new_hmac("yeet".to_string());
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None,
            server_lib_root: None,
            environment: RunningEnvironment::default(),
            repository: RepositoryType::Fake,
        };

        let state = State::new(conf);
        let filter = jwt_filter::<Uuid>(&state);
        assert!(!warp::test::request().matches(&filter))
    }

    #[test]
    fn pass_user_filter() {
        let secret = Secret::new_hmac("yeet".to_string());
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None,
            server_lib_root: None,
            environment: RunningEnvironment::default(),
            repository: RepositoryType::Fake,
        };
        let state = State::new(conf);
        let uuid = Uuid::new_v4();
        let user = User {
            uuid,
            google_user_id: "yeet".to_string(),
            google_name: None,
        };
        let jwt: JwtPayload<User> = JwtPayload::new(user, Duration::weeks(2));
        let jwt = jwt.encode_jwt_string(&secret).unwrap();

        let filter = user_filter(&state);

        assert!(warp::test::request()
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .matches(&filter))
    }

    #[test]
    fn pass_optional_user_filter_empty() {
        let secret = Secret::new_hmac("yeet".to_string());
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None,
            server_lib_root: None,
            environment: RunningEnvironment::default(),
            repository: RepositoryType::Fake,
        };

        let state = State::new(conf);
        let filter = optional_user_filter(&state);
        assert!(warp::test::request().matches(&filter))
    }

    #[test]
    fn pass_optional_user_filter_with_jwt() {
        let secret = Secret::new_hmac("yeet".to_string());
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None,
            server_lib_root: None,
            environment: RunningEnvironment::default(),
            repository: RepositoryType::Fake,
        };
        let state = State::new(conf);
        let uuid = Uuid::new_v4();
        let jwt = JwtPayload::new(uuid, Duration::weeks(2));
        let jwt = jwt.encode_jwt_string(&secret).unwrap();

        let filter = optional_user_filter(&state);

        assert!(warp::test::request()
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .matches(&filter))
    }

    #[test]
    fn reject_outdated() {
        let mut payload = JwtPayload::new("hello_there".to_string(), Duration::weeks(-1)); // Expire a week ago
        payload.iat = (chrono::Utc::now() - Duration::weeks(2)).naive_utc(); // "issued at" 2 weeks ago

        let secret = Secret::new_hmac("yeet".to_string());
        let encoded = payload.encode_jwt_string(&secret).unwrap();
        let header_string = format!("{} {}", BEARER, encoded);

        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None,
            server_lib_root: None,
            environment: RunningEnvironment::default(),
            repository: RepositoryType::Fake,
        };
        let state = State::new(conf);

        let filter = jwt_filter::<String>(&state);

        assert!(!warp::test::request()
            .header(AUTHORIZATION_HEADER_KEY, header_string)
            .matches(&filter));
    }
}

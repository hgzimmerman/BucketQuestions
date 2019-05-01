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
use apply::Apply;
use authorization::{JwtPayload, Secret, AUTHORIZATION_HEADER_KEY};
use egg_mode::{KeyPair, Token};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Rejection};
use std::env;

/// A serializeable variant of Egg-mode's Token::Access variant
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwitterToken {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

impl From<Token> for TwitterToken {
    fn from(value: Token) -> Self {
        match value {
            Token::Access { consumer, access } => TwitterToken {
                consumer_key: consumer.key.to_string(),
                consumer_secret: consumer.secret.to_string(),
                access_key: access.key.to_string(),
                access_secret: access.secret.to_string(),
            },
            _ => panic!("No support for non-access tokens"),
        }
    }
}

impl Into<Token> for TwitterToken {
    fn into(self) -> Token {
        Token::Access {
            consumer: KeyPair {
                key: self.consumer_key.into(),
                secret: self.consumer_secret.into(),
            },
            access: KeyPair {
                key: self.access_key.into(),
                secret: self.access_secret.into(),
            },
        }
    }
}

/// An application-specific subject section for use within a JWT
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
    pub user_uuid: Uuid,
    pub twitter_token: TwitterToken,
}



//use oauth2::basic::BasicClient;
use oauth2::prelude::*;
use oauth2::{
    basic::BasicClient,
    AuthUrl,
    AuthorizationCode,
    ClientId,
    ClientSecret,
    CsrfToken,
    RedirectUrl,
    Scope,
    TokenUrl,
};
use url::Url;


/// Gets the login link for Google OAuth.
fn get_google_login_link(redirect_url: Url) -> Url {
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
//        .add_scope(Scope::new(
//            "https://www.googleapis.com/auth/calendar".to_string(),
//        ))
//        .add_scope(Scope::new(
//            "https://www.googleapis.com/auth/plus.me".to_string(),
//        ))
        .set_redirect_url(RedirectUrl::new(redirect_url));

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
        .and_then(|bearer_string, secret| {
            JwtPayload::extract_jwt(bearer_string, &secret)
                .and_then(JwtPayload::validate_dates)
                .map_err(Error::AuthError)
                .map_err(Error::reject)
        })
        .boxed()
}

/// Brings the secret into scope.
/// The secret is used to create and verify JWTs.
///
/// # Arguments
/// * secret - The secret to be made available by the returned Filter.
pub fn secret_filter(
    secret: Secret,
) -> impl Filter<Extract = (Secret,), Error = Rejection> + Clone {
    warp::any().and_then(move || -> Result<Secret, Rejection> { Ok(secret.clone()) })
}

/// If the user has a JWT, then the user has basic user privileges.
///
/// # Arguments
/// * s - The state used to validate the JWT
pub fn user_filter(s: &State) -> BoxedFilter<(Uuid,)> {
    warp::any()
        .and(jwt_filter(s))
        .map(JwtPayload::subject)
        .map(|subject: Subject| subject.user_uuid)
        .boxed()
}

#[allow(dead_code)]
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

pub fn twitter_token_filter(s: &State) -> BoxedFilter<(Token,)> {
    warp::any()
        .and(jwt_filter(s))
        .map(JwtPayload::<Subject>::subject)
        .map(|subject: Subject| subject.twitter_token.apply(TwitterToken::into))
        .boxed()
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use crate::state::StateConfig;
    use authorization::BEARER;
    use chrono::Duration;

    #[test]
    fn pass_filter() {
        let secret = Secret::new("yeet");
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None,
            server_lib_root: None,
            is_production: false,
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
    fn does_not_pass_filter() {
        let secret = Secret::new("yeet");
        let conf = StateConfig {
            secret: Some(secret.clone()),
            max_pool_size: None,
            server_lib_root: None,
            is_production: false,
        };

        let state = State::new(conf);
        let filter = jwt_filter::<Uuid>(&state);
        assert!(!warp::test::request().matches(&filter))
    }

}

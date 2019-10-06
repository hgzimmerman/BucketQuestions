//! Represents the shared server resources that all requests may utilize.
pub mod state_config;
#[cfg(test)]
pub mod test_util;

use crate::{
    error::Error, server_auth::create_google_oauth_client, state::state_config::StateConfig,
};
use apply::Apply;
use authorization::Secret;
use db::{Repository, RepositoryProvider};
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Client,
};
use hyper_tls::HttpsConnector;
use oauth2::basic::BasicClient;
use pool::{init_pool, PoolConfig, DATABASE_URL};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    fmt::{Debug, Formatter},
    path::PathBuf,
};
use url::Url;
use warp::{Filter, Rejection};
use crate::config::RepositoryType;
use db::fake::FakeDatabase;
use std::sync::{Mutex, Arc};

/// Simplified type for representing a HttpClient.
pub type HttpsClient = Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

/// State that is passed around to all of the api handlers.
/// It can be used to acquire connections to the database,
/// or to reference the key that signs the access tokens.
///
/// These entities are acquired by running a filter function that brings them
/// into the scope of the relevant api.
pub struct State {
    /// The provider that can provide repositories.
    repository_provider: RepositoryProvider,
    /// The secret key.
    secret: Secret,
    /// Https client
    https: HttpsClient,
    /// The client for operating with google oauth tokens
    google_oauth_client: BasicClient,
    /// The path to the server directory.
    /// This allows file resources to have a common reference point when determining from where to serve assets.
    server_lib_root: PathBuf,
    /// Redirect url for Oauth
    redirect_url: Url,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        f.debug_struct("State")
            .field("repository_provider", &self.repository_provider)
            .field("secret", &self.secret) // Display for Secret self-censors
            .field("server_lib_root", &self.server_lib_root)
            .field("redirect_url", &self.redirect_url)
            .finish()
    }
}

impl State {
    /// Creates a new state.
    pub fn new(conf: StateConfig) -> Self {
        const RANDOM_KEY_LENGTH: usize = 200;
        let secret = conf.secret.unwrap_or_else(|| {
            // Generate a new random key if none is provided.
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(RANDOM_KEY_LENGTH)
                .collect::<String>()
                .apply(|s| Secret::new_hmac(s))
        });

        let pool_conf = PoolConfig {
            max_connections: conf.max_pool_size,
            ..Default::default()
        };

        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, _>(https);

        let redirect_url = conf.environment.create_redirect_url();
        let google_oauth_client = create_google_oauth_client(redirect_url.clone());

        let root = conf.server_lib_root.unwrap_or_else(|| PathBuf::from("./"));


        let repository_provider = match conf.repository {
            RepositoryType::Fake => RepositoryProvider::Fake(Arc::new(Mutex::new(FakeDatabase::default()))),
            RepositoryType::Database => RepositoryProvider::Pool(init_pool(DATABASE_URL, pool_conf))
        };

        State {
            repository_provider,
            secret,
            https: client,
            google_oauth_client,
            server_lib_root: root,
            redirect_url,
        }
    }

    /// Gets an abstract repository object.
    /// This can be used to access a backing store for the application.
    pub fn db(
        &self,
    ) -> impl Filter<Extract = (Box<dyn Repository + Send + 'static>,), Error = Rejection> + Clone
    {
        let r = self.repository_provider.clone();
        warp::any().and_then(
            move || -> Result<Box<dyn Repository + Send + 'static>, Rejection> {
                r.get_repo().map_err(|_| {
                    log::error!("Pool exhausted: Could not get database connection.");
                    Error::DatabaseUnavailable.reject()
                })
            },
        )
    }

    /// Gets the secret used for authoring JWTs
    pub fn secret(&self) -> impl Filter<Extract = (Secret,), Error = Rejection> + Clone {
        let secret = self.secret.clone();
        warp::any().and_then(move || -> Result<Secret, Rejection> { Ok(secret.clone()) })
    }

    /// Gets the https client used for making dependent api calls.
    pub fn https_client(&self) -> impl Filter<Extract = (HttpsClient,), Error = Rejection> + Clone {
        let client = self.https.clone();
        // This needs to be able to return a Result w/a Rejection, because there is no way to specify the type of
        // warp::never::Never because it is private, precluding the possibility of using map instead of and_then().
        // This adds space overhead, but not nearly as much as using a boxed filter.
        warp::any().and_then(move || -> Result<HttpsClient, Rejection> { Ok(client.clone()) })
    }

    /// Gets the client for contacting Google OAuth.
    pub fn google_client(
        &self,
    ) -> impl Filter<Extract = (BasicClient,), Error = Rejection> + Clone {
        let client = self.google_oauth_client.clone();
        warp::any().and_then(move || -> Result<BasicClient, Rejection> { Ok(client.clone()) })
    }

    /// Gets the root of the server.
    pub fn server_lib_root(&self) -> PathBuf {
        self.server_lib_root.clone()
    }

    /// Gets the URL that auth should redirect to after the user has granted access.
    pub fn redirect_url(&self) -> Url {
        self.redirect_url.clone()
    }
}

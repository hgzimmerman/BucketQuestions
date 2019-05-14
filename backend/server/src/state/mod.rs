//! Represents the shared server resources that all requests may utilize.
pub mod state_config;

use crate::{error::Error, server_auth::secret_filter};

use crate::server_auth::create_google_oauth_client;
use apply::Apply;
use authorization::Secret;
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
use db::{Repository, RepositoryProvider};
#[cfg(test)]
use db::test::{setup_mock_provider};
use crate::state::state_config::StateConfig;

/// Simplified type for representing a HttpClient.
pub type HttpsClient = Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

/// State that is passed around to all of the api handlers.
/// It can be used to acquire connections to the database,
/// or to reference the key that signs the access tokens.
///
/// These entities are acquired by running a filter function that brings them
/// into the scope of the relevant api.
pub struct State {
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
            .field("secret", &self.secret)
            .field("https", &"https client".to_owned())
            .field("google_oauth_client", &"Google Oauth Client".to_owned())
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
                .apply(|s| Secret::new(&s))
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

        let repository_provider = RepositoryProvider::Pool(init_pool(DATABASE_URL, pool_conf));

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
    pub fn db2(&self) -> impl Filter<Extract = (Box<dyn Repository + Send + 'static>, ), Error = Rejection> + Clone{
        let r = self.repository_provider.clone();
        warp::any()
            .and_then(move || -> Result<Box<Repository + Send + 'static>, Rejection> {
                r.get_repo()
                    .map_err(|_| {
                        log::error!("Pool exhausted: could not get database connection.");
                        Error::DatabaseUnavailable.reject()
                    })
            })
    }

    /// Gets the secret used for authoring JWTs
    pub fn secret(&self) -> impl Filter<Extract = (Secret,), Error = Rejection> + Clone {
        secret_filter(self.secret.clone())
    }

    /// Gets the https client used for making dependent api calls.
    pub fn https_client(&self) -> impl Filter<Extract = (HttpsClient,), Error = Rejection> + Clone {
        /// Function that creates the HttpClient filter.
        fn http_filter(
            client: HttpsClient,
        ) -> impl Filter<Extract = (HttpsClient,), Error = Rejection> + Clone {
            // This needs to be able to return a Result w/a Rejection, because there is no way to specify the type of
            // warp::never::Never because it is private, precluding the possibility of using map instead of and_then().
            // This adds space overhead, but not nearly as much as using a boxed filter.
            warp::any().and_then(move || -> Result<HttpsClient, Rejection> { Ok(client.clone()) })
        }
        http_filter(self.https.clone())
    }

    pub fn google_client(
        &self,
    ) -> impl Filter<Extract = (BasicClient,), Error = Rejection> + Clone {
        fn client_filter(
            client: BasicClient,
        ) -> impl Filter<Extract = (BasicClient,), Error = Rejection> + Clone {
            // This needs to be able to return a Result w/a Rejection, because there is no way to specify the type of
            // warp::never::Never because it is private, precluding the possibility of using map instead of and_then().
            // This adds space overhead, but not nearly as much as using a boxed filter.
            warp::any().and_then(move || -> Result<BasicClient, Rejection> { Ok(client.clone()) })
        }
        client_filter(self.google_oauth_client.clone())
    }

    pub fn server_lib_root(&self) -> PathBuf {
        self.server_lib_root.clone()
    }

    pub fn redirect_url(&self) -> Url {
        self.redirect_url.clone()
    }


}

#[cfg(test)]
pub mod test_util {
    use super::*;
    use db::test::fixture::Fixture;
    use crate::state::state_config::RunningEnvironment;
    use db::test::execute_pool_test;

    impl State {
        /// Creates a new state object from an existing object pool.
        /// This is useful if using fixtures.
        #[cfg(test)]
        pub fn testing_init(repository_provider: RepositoryProvider, secret: Secret) -> Self {
            use std::time::Duration;
            let https = HttpsConnector::new(1).unwrap();
            let client = Client::builder()
                .keep_alive_timeout(Some(Duration::new(12, 0)))
                .build::<_, Body>(https);
            let redirect_url = RunningEnvironment::Staging { port: 8080 }.create_redirect_url();
            let google_oauth_client = create_google_oauth_client(redirect_url.clone());


            State {
                repository_provider,
                secret,
                https: client,
                google_oauth_client,
                server_lib_root: PathBuf::from("./"), // THIS makes the assumption that the tests are run from the backend/server dir.
                redirect_url,
            }
        }
    }

    /// This executes a test function on a repository that has been created in a manner
    /// that it is guaranteed not to interfere with other tests.
    #[cfg(test)]
    pub fn execute_test_on_repository<Fix, Fun>(f: Fun)
    where
        Fix: Fixture,
        Fun: Fn(&Fix, RepositoryProvider),
    {
        // TODO, remove the feature flag and just use a cfg value.
        if cfg!(feature = "integration") {
            execute_pool_test(f)
        } else {
            let (fixture, mock): (Fix, RepositoryProvider) = setup_mock_provider();
            f(&fixture, mock)
        }
    }
}

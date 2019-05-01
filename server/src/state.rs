//! Represents the shared server resources that all requests may utilize.
use crate::{error::Error, server_auth::secret_filter};

use apply::Apply;
use authorization::Secret;
use egg_mode::KeyPair;
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Client,
};
use hyper_tls::HttpsConnector;
use pool::{init_pool, Pool, PoolConfig, PooledConn, DATABASE_URL};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::path::PathBuf;
use warp::{Filter, Rejection};

/// Simplified type for representing a HttpClient.
pub type HttpsClient = Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

/// State that is passed around to all of the api handlers.
/// It can be used to acquire connections to the database,
/// or to reference the key that signs the access tokens.
///
/// These entities are acquired by running a filter function that brings them
/// into the scope of the relevant api.
pub struct State {
    /// A pool of database connections.
    database_connection_pool: Pool,
    /// The secret key.
    secret: Secret,
    /// Https client
    https: HttpsClient,
    /// Twitter consumer token
    twitter_consumer_token: KeyPair,
    /// The path to the server directory.
    /// This allows file resources to have a common reference point when determining from where to serve assets.
    server_lib_root: PathBuf,
    /// Is the server running in a production environment
    is_production: bool,
}

/// Configuration object for creating the state.
///
/// If unspecified, it will default to a sane default.
#[derive(Debug, Default)]
pub struct StateConfig {
    pub secret: Option<Secret>,
    pub max_pool_size: Option<u32>,
    pub server_lib_root: Option<PathBuf>,
    pub is_production: bool,
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

        let pool = init_pool(DATABASE_URL, pool_conf);
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, _>(https);

        let twitter_con_token = get_twitter_con_token();

        let root = conf.server_lib_root.unwrap_or_else(|| PathBuf::from("./"));

        State {
            database_connection_pool: pool, //db_filter(pool),
            secret,
            https: client,
            twitter_consumer_token: twitter_con_token.clone(),
            server_lib_root: root,
            is_production: conf.is_production,
        }
    }

    /// Gets a pooled connection to the database.
    pub fn db(&self) -> impl Filter<Extract = (PooledConn,), Error = Rejection> + Clone {
        /// Filter that exposes connections to the database to individual filter requests
        fn db_filter(pool: Pool) -> impl Filter<Extract = (PooledConn,), Error = Rejection> + Clone {
            fn get_conn_from_pool(pool: &Pool) -> Result<PooledConn, Rejection> {
                pool.clone()
                    .get() // Will get the connection from the pool, or wait a specified time until one becomes available.
                    .map_err(|_| {
                        log::error!("Pool exhausted: could not get database connection.");
                        Error::DatabaseUnavailable.reject()
                    })
            }

            warp::any().and_then(move || -> Result<PooledConn, Rejection> { get_conn_from_pool(&pool) })
        }

        db_filter(self.database_connection_pool.clone())
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

    /// Access the twitter consumer token.
    pub fn twitter_consumer_token(&self) -> impl Filter<Extract = (KeyPair,), Error = Rejection> + Clone {
        fn twitter_consumer_token_filter(twitter_consumer_token: KeyPair) -> impl Filter<Extract = (KeyPair,), Error = Rejection> + Clone {
            warp::any().and_then(move || -> Result<KeyPair, Rejection> { Ok(twitter_consumer_token.clone()) })
        }
        twitter_consumer_token_filter(self.twitter_consumer_token.clone())
    }


    pub fn server_lib_root(&self) -> PathBuf {
        self.server_lib_root.clone()
    }

    pub fn is_production(&self) -> bool {
        self.is_production
    }

    /// Creates a new state object from an existing object pool.
    /// This is useful if using fixtures.
    #[cfg(test)]
    pub fn testing_init(pool: Pool, secret: Secret) -> Self {
        use std::time::Duration;
        let https = HttpsConnector::new(1).unwrap();
        let client = Client::builder()
            .keep_alive_timeout(Some(Duration::new(12, 0)))
            .build::<_, Body>(https);

        let twitter_con_token = get_twitter_con_token();

        State {
            database_connection_pool: pool,
            secret,
            https: client,
            twitter_consumer_token: twitter_con_token,
            server_lib_root: PathBuf::from("./"), // THIS makes the assumption that the tests are run from the backend/server dir.
            is_production: false,
        }
    }
}




/// Gets the connection key pair for the serer.
/// This represents the authenticity of the application
fn get_twitter_con_token() -> KeyPair {
    // TODO move getting these into a config object, or get them directly from the filesystem.
    // These definitely shouldn't be in source code, but I don't care,
    // I just want this to work right now. Also, this is a school project.
    const KEY: &str = "Pq2sA4Lfbovd4SLQhSQ6UPEVg";
    const SECRET: &str = "uK6U7Xqj2QThlm6H3y8dKSH3itZgpo9AVhR5or80X9umZc62ln";

    egg_mode::KeyPair::new(KEY, SECRET)
}

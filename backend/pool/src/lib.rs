//! Crate for abstracting over r2d2's pool implementation.
//! This is needed so that the db, server, and testing crates can all have a common pool abstraction to work with.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

use apply::Apply;
use diesel::{pg::PgConnection, r2d2::ConnectionManager, Connection};
use r2d2::{Pool as R2D2Pool, PooledConnection};
use std::time::Duration;

/// The URL used for connecting to the database.
/// Sourced via an environment variable.
/// It should be in the postgres format.
pub const DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Holds a bunch of connections to the database and hands them out to routes as needed.
pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
/// A Database connection that is lent out from the pool.
/// It can dereference to a PgConnection where needed.
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

/// Configuration object for the pool.
#[derive(Clone, Copy, Default, Debug)]
pub struct PoolConfig {
    /// The maximum number of connections.
    pub max_connections: Option<u32>,
    /// The minimum number of connections.
    pub min_connections: Option<u32>,
    /// Max lifetime in minutes.
    pub max_lifetime: Option<Duration>,
    /// The time for how long a acquisition of a pooled connection will wait until it fails.
    pub connection_timeout: Option<Duration>,
}

/// Initializes the pool.
///
/// # Arguments
/// * db_url - The url that the pool will use to establish connections.
/// * conf - The configuration object.
pub fn init_pool(db_url: &str, conf: PoolConfig) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let builder = r2d2::Pool::builder()
        .apply(|builder| {
            if let Some(max_size) = conf.max_connections {
                builder.max_size(max_size)
            } else {
                builder
            }
        })
        .apply(|builder| builder.min_idle(conf.min_connections))
        .apply(|builder| {
            if let Some(max_lifetime) = conf.max_lifetime {
                builder.max_lifetime(Some(max_lifetime))
            } else {
                builder
            }
        })
        .apply(|builder| {
            if let Some(timeout) = conf.connection_timeout {
                builder.connection_timeout(timeout)
            } else {
                builder
            }
        });

    builder
        .build(manager)
        .expect("Could not initialize DB Pool")
}

/// Create a single connection.
///
/// This may be useful for testing.
pub fn create_single_connection(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url)
        .expect("Database not available. Maybe provided url is wrong, or database is down?")
}

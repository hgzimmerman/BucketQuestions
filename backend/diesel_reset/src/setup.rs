use crate::reset::{reset_database, run_migrations};
use diesel::{Connection, PgConnection};
use pool::{init_pool, Pool, PoolConfig};

use std::sync::{Mutex, MutexGuard};

pub const DATABASE_NAME: &str = env!("TEST_DATABASE_NAME");

/// Points to the database that tests will be performed on.
/// The database schema will be destroyed and recreated before every test.
/// It absolutely should _never_ point to a production database,
/// as tests ran using it will likely create an admin account that has known login credentials.
pub const DATABASE_URL: &str = env!("TEST_DATABASE_URL");

/// Should point to the base postgres account.
/// One that has authority to create and destroy other databases.
const DROP_DATABASE_URL: &str = env!("DROP_DATABASE_URL");

// This creates a singleton of the base database connection.
//
// The base database connection is required, because you cannot drop the other database from itself.
//
// Because it is wrapped in a mutex, only one test at a time can access it.
// The setup method will lock it and use it to reset the database.
//
// It is ok if a test fails and poisons the mutex, as the one place where it is used disregards the poison.
// Disregarding the poison is fine because code using the mutex-ed value never modifies the value,
// so there is no indeterminate state to contend with if a prior test has panicked.
lazy_static! {
    static ref CONN: Mutex<PgConnection> = Mutex::new(
        PgConnection::establish(DROP_DATABASE_URL).expect("Administration database not available")
    );
}

const MIGRATIONS_DIRECTORY: &str = "../db/migrations";

#[deprecated]
pub fn setup_pool() -> Pool {
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn, DATABASE_NAME);

    // Establish a pool, this will be passed in as part of the State object when simulating the api.
    let pool_conf = PoolConfig {
        max_connections: Some(2),
        min_connections: Some(1),
        max_lifetime: None,
        connection_timeout: None,
    };
    init_pool(DATABASE_URL, pool_conf)
}

/// Sole purpose is opaquely containing a lock on the admin connection.
/// This keeps the global mutex locked, and prevents tests from clobbering each other
/// by resetting each other's databases.
pub struct AdminLock<'a>(MutexGuard<'a, PgConnection>);

pub fn setup_pool_sequential<'a>() -> (Pool, AdminLock<'a>) {
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn, DATABASE_NAME);
    // Establish a pool, this will be passed in as part of the State object when simulating the api.
    let pool_conf = PoolConfig {
        // Apparently, if the pool size is too small, then the tests might time out.
        // 2 is too small, 5 works reliably under normal circumstances
        max_connections: Some(5),
        min_connections: Some(1),
        max_lifetime: None,
        connection_timeout: None,
    };
    let pool = init_pool(DATABASE_URL, pool_conf);
    run_migrations(&pool.get().unwrap(), MIGRATIONS_DIRECTORY);
    (pool, AdminLock(admin_conn))
}

//use diesel::r2d2::ConnectionManager;
//use diesel::connection::TransactionManager;
//pub fn setup_pool_no_internal_dependencies<'a, C>() -> (r2d2::Pool<ConnectionManager<C>>, AdminLock<'a>)
//where
//    C: Connection<TransactionManager=diesel::connection::AnsiTransactionManager> + 'static,
//    C::Backend: diesel::backend::UsesAnsiSavepointSyntax
//        + diesel::connection::TransactionManager<C>
//        + diesel::backend::SupportsDefaultKeyword,
//{
//    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
//        Ok(guard) => guard,
//        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
//    };
//    reset_database(&admin_conn, DATABASE_NAME);
//    // Establish a pool, this will be passed in as part of the State object when simulating the api.
//
//
//    let manager = ConnectionManager::<C>::new(DATABASE_URL);
//
//    let mut builder = r2d2::Pool::builder();
//    let builder = builder.max_size(2);
//    let pool = builder.build(manager).expect("Could not build pool");
//    run_migrations(&pool.get().unwrap(), MIGRATIONS_DIRECTORY);
//    (pool, AdminLock(admin_conn) )
//}

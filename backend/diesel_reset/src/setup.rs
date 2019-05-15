use crate::reset::{reset_database, run_migrations};
use diesel::{Connection, PgConnection, r2d2};
//use pool::{init_pool, Pool, PoolConfig};

use std::sync::{Mutex, MutexGuard};

pub const DATABASE_NAME: &str = env!("TEST_DATABASE_NAME");

/// Points to the database that tests will be performed on.
/// The database schema will be destroyed and recreated before every test.
/// It absolutely should _never_ point to a production database,
/// as tests ran using it will likely create an admin account that has known login credentials.
pub const DATABASE_URL: &str = env!("TEST_DATABASE_URL");

/// Should point to the base postgres account.
/// One that has authority to create and destroy other databases.
pub const DROP_DATABASE_URL: &str = env!("DROP_DATABASE_URL");

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

pub const MIGRATIONS_DIRECTORY: &str = "../db/migrations";


/// Sole purpose is opaquely containing a lock on the admin connection.
/// This keeps the global mutex locked, and prevents tests from clobbering each other
/// by resetting each other's databases.
pub struct AdminLock<'a>(MutexGuard<'a, PgConnection>);


use diesel::r2d2::ConnectionManager;

pub fn setup_pool_sequential<'a>() -> (r2d2::Pool<ConnectionManager<PgConnection>>, AdminLock<'a>)
{
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn, DATABASE_NAME);

    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);

    let builder = r2d2::Pool::builder()
        .max_size(5)
        .min_idle(Some(2));
    let pool = builder.build(manager).expect("Could not build pool");
    run_migrations(&pool.get().unwrap(), MIGRATIONS_DIRECTORY);
    (pool, AdminLock(admin_conn) )
}

/// Cleanup wrapper
pub struct Cleanup(PgConnection, String);

impl Drop for Cleanup {
    fn drop(&mut self) {
        crate::reset::drop_database(&self.0, &self.1)
            .expect("Couldn't drop database at end of test.");
    }
}

// TODO determine if this works
/// Creates a random db using the admin_db, then deletes it when the test finishes
pub fn setup_pool_random_db(admin_conn: PgConnection, url_part: &str, migrations_directory: &str) -> (r2d2::Pool<ConnectionManager<PgConnection>>, Cleanup) {
    let db_name = nanoid::simple(); // Gets a random url-safe string.
    crate::reset::create_database(&admin_conn, &db_name).expect("Couldn't create database");

    let url = format!("{}/{}", url_part, db_name);
    let manager = ConnectionManager::<PgConnection>::new(url);

    let pool = r2d2::Pool::builder()
        .max_size(5)
        .min_idle(Some(2))
        .build(manager)
        .expect("Couldn't create pool");

    run_migrations(&pool.get().unwrap(), migrations_directory);

    let cleanup = Cleanup(admin_conn, db_name);
    (pool, cleanup)
}

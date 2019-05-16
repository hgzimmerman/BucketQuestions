use crate::reset::{run_migrations};
use diesel::{r2d2, PgConnection};
#[cfg(test)]
use diesel::Connection;

/// The origin (scheme, user, password, address, port) of the test database.
///
/// This determines which database server is connected to, but allows for specification of
/// a specific database instance within the server to connect to and run tests with.
#[cfg(test)]
pub const DATABASE_ORIGIN: &str = env!("TEST_DATABASE_ORIGIN");

/// Should point to the base postgres account.
/// One that has authority to create and destroy other database instances.
///
/// It is expected to be on the same database server as the one indicated by DATABASE_ORIGIN.
pub const DROP_DATABASE_URL: &str = env!("DROP_DATABASE_URL");


pub const MIGRATIONS_DIRECTORY: &str = "../db/migrations";

use diesel::r2d2::ConnectionManager;

/// Cleanup wrapper.
/// Contains the admin connection and the name of the database (not the whole url).
pub struct Cleanup(PgConnection, String);

impl Drop for Cleanup {
    fn drop(&mut self) {
        crate::reset::drop_database(&self.0, &self.1)
            .expect("Couldn't drop database at end of test.");
    }
}

/// Creates a random db using the admin_db, then deletes it when the test finishes
pub fn setup_pool_random_db(
    admin_conn: PgConnection,
    url_part: &str,
    migrations_directory: &str,
) -> (r2d2::Pool<ConnectionManager<PgConnection>>, Cleanup) {
    let db_name = nanoid::simple(); // Gets a random url-safe string.
    // delegate logic to this function
    setup_pool_named_db(admin_conn, url_part, migrations_directory, db_name)
}

/// Utility function that creates a database with a known name and runs migrations on it.
///
/// # Note
/// This function exists to facilitate verification that that the database is still dropped
/// even if a test panics.
fn setup_pool_named_db(
    admin_conn: PgConnection,
    url_part: &str,
    migrations_directory: &str,
    db_name: String
) -> (r2d2::Pool<ConnectionManager<PgConnection>>, Cleanup) {
    // This makes the assumption that the provided database name does not already exist on the system.
    crate::reset::create_database(&admin_conn, &db_name).expect("Couldn't create database");

    let url = format!("{}/{}", url_part, db_name);
    let manager = ConnectionManager::<PgConnection>::new(url);

    let pool = r2d2::Pool::builder()
        .max_size(3)
        .min_idle(Some(2))
        .build(manager)
        .expect("Couldn't create pool");

    run_migrations(&pool.get().unwrap(), migrations_directory);

    let cleanup = Cleanup(admin_conn, db_name);
    (pool, cleanup)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cleanup_drops_db_after_panic() {
        let url_origin = DATABASE_ORIGIN;

        let db_name= "cleanup_drops_db_after_panic_TEST_DB".to_string();

        std::panic::catch_unwind(|| {
            let admin_conn = PgConnection::establish(DROP_DATABASE_URL).expect("Should be able to connect to admin db");
            let _ = setup_pool_named_db(admin_conn, url_origin, "../db/migrations", db_name.clone());
            panic!("expected_panic");
        })
            .expect_err("Should catch panic.");

        let admin_conn = PgConnection::establish(DROP_DATABASE_URL).expect("Should be able to connect to admin db");
        let database_exists: bool = crate::reset::pg_database_exists(&admin_conn, &db_name)
            .expect("Should determine if database exists");
        assert!(!database_exists)
    }
}


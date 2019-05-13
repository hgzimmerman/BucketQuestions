use crate::{
    database_error::{DatabaseError, DatabaseResult},
//    fixture::Fixture,
    query_helper,
};
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, QueryResult,
    RunQueryDsl,
};
use migrations_internals as migrations;
use pool::{Pool, PoolConfig, PooledConn, init_pool};

use std::sync::{Mutex, MutexGuard};
use std::ops::Deref;

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
    static ref CONN: Mutex<PgConnection> =
        Mutex::new(PgConnection::establish(DROP_DATABASE_URL).expect("Database not available"));
}

//
//
//// TODO I don't think that this function can be in this crate.
//// I think it needs to be in the same crate as the Repository trait
///// Resets the database and the given future and provides a pool that can be used to construct a state used in warp.
//pub fn setup_warp<Fun, Fix>(mut test_function: Fun)
//where
//    Fun: FnMut(&Fix, Pool),
//    Fix: Fixture<Repository=PgConnection>,
//{
//    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
//        Ok(guard) => guard,
//        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
//    };
//    reset_database(&admin_conn);
//
//    // Establish a pool, this will be passed in as part of the State object when simulating the api.
//    let testing_pool = pool::init_pool(DATABASE_URL, PoolConfig::default());
//
//    let conn: PgConnection = PgConnection::establish(DATABASE_URL)
//        .expect("Database not available.");
//    let fixture = Fix::generate(&conn);
//    test_function(&fixture, testing_pool)
//}


#[deprecated]
pub fn setup_pool() -> Pool {
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn);

    // Establish a pool, this will be passed in as part of the State object when simulating the api.
    let pool_conf = PoolConfig {
        max_connections: Some(2),
        min_connections: Some(1),
        max_lifetime: None,
        connection_timeout: None
    };
    init_pool(DATABASE_URL, pool_conf)
}

/// Sole purpose is opaquely containing a lock on the admin connection.
pub struct AdminLock<'a>(MutexGuard<'a, PgConnection>);

pub fn setup_pool_sequential<'a>() -> (Pool, AdminLock<'a>) {
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn);
    // Establish a pool, this will be passed in as part of the State object when simulating the api.
    let pool_conf = PoolConfig {
        max_connections: Some(10),
        min_connections: Some(1),
        max_lifetime: None,
        connection_timeout: None
    };
    let pool = init_pool(DATABASE_URL, pool_conf);
    run_migrations(&pool.get().unwrap());
    (pool, AdminLock(admin_conn) )
}



// TODO, this seems unsound. I would imagine for some tests, the database could be reset mid-test due to the lack of locks.
// This doesn't seem to happen.
#[deprecated]
pub fn setup_single_connection() -> PgConnection {
    let admin_conn: MutexGuard<PgConnection> = match CONN.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Don't care if the mutex is poisoned
    };
    reset_database(&admin_conn);

    let conn: PgConnection = PgConnection::establish(DATABASE_URL)
        .expect("Database not available.");

    run_migrations(&conn);
    conn
}

/// Drops the database and then recreates it.
/// The guarantee that this function provides is that the test database will be in a default
/// state, without any run migrations after this ran.
fn reset_database(conn: &PgConnection) {
    drop_database(&conn).expect("Could not drop db");
    create_database(&conn).expect("Could not create Database");
}

/// Drops the database, completely removing every table (and therefore every row) in the database.
fn drop_database(conn: &PgConnection) -> DatabaseResult<()> {
    if pg_database_exists(&conn, DATABASE_NAME)? {
        println!("Dropping database: {}", DATABASE_NAME);
        query_helper::drop_database(DATABASE_NAME)
            .if_exists()
            .execute(conn)
            .map_err(DatabaseError::from)
            .map(|_| ())
    } else {
        Ok(())
    }
}

/// Recreates the database.
fn create_database(conn: &PgConnection) -> DatabaseResult<()> {
    let db_result = query_helper::create_database(DATABASE_NAME)
        .execute(conn)
        .map_err(DatabaseError::from)
        .map(|_| ());
    println!("Created database:  {}", DATABASE_NAME);
    db_result
}

/// Creates tables in the database.
fn run_migrations(conn: &PgConnection) {
    use std::path::Path;
    // This directory traversal allows this library to be used by any crate in the `backend` crate.
    const MIGRATIONS_DIRECTORY: &str = "../db/migrations";

    let migrations_dir: &Path = Path::new(MIGRATIONS_DIRECTORY);
    migrations::run_pending_migrations_in_directory(conn, migrations_dir, &mut ::std::io::sink())
        .expect("Could not run migrations.");
    println!("Ran migrations:    {}", DATABASE_NAME);
}

table! {
    pg_database (datname) {
        datname -> Text,
        datistemplate -> Bool,
    }
}

/// Convenience function used when dropping the database.
fn pg_database_exists(conn: &PgConnection, database_name: &str) -> QueryResult<bool> {
    use self::pg_database::dsl::*;

    pg_database
        .select(datname)
        .filter(datname.eq(database_name))
        .filter(datistemplate.eq(false))
        .get_result::<String>(conn)
        .optional()
        .map(|x| x.is_some())
}

//! Test module for convienence functions and fixtures
pub mod answer_fixture;
pub mod bucket_and_user_fixture;
pub mod bucket_fixture;
pub mod bucket_user_relation_fixture;
pub mod empty_fixture;
pub mod fixture;
pub mod question_fixture;
pub mod user_fixture;

use self::fixture::Fixture;
use crate::{mock::MockDatabase, BoxedRepository, RepositoryProvider};
use diesel::PgConnection;
use diesel_reset::setup::{setup_pool_random_db, Cleanup, DROP_DATABASE_URL, MIGRATIONS_DIRECTORY};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

/// Determines what set of tests will run for test executors that rely on this.
#[derive(Debug, Clone, Copy)]
pub enum TestType {
    /// Unit tests will run against a mock object
    Unit,
    /// Integration tests will run against a test database
    Integration,
    /// Both types of tests will run
    Both,
}
impl TestType {
    /// Gets the test type from an environment variable
    pub fn get_test_type_from_env() -> Self {
        let test_type = std::env::var("TEST_TYPE")
            .expect("TEST_TYPE env variable should be specified to be either 'unit', 'integration', or 'both'.");
        match test_type.to_lowercase().deref() {
            "unit" => TestType::Unit,
            "integration" => TestType::Integration,
            "both" => TestType::Both,
            x => panic!(
                "Invalid test type: {}. \n Must be 'unit', 'integration', or 'both'",
                x
            ),
        }
    }
}

/// Execute a test based on what testing environment you want.
pub fn execute_test<Fix, Fun>(f: Fun)
where
    Fix: Fixture,
    Fun: Fn(&Fix, BoxedRepository),
{
    match TestType::get_test_type_from_env() {
        TestType::Unit => {
            let (fix, repo) = setup_mock::<Fix>();
            f(&fix, repo);
        }
        TestType::Integration => {
            let (fix, repo, _cleanup_wrapper) = setup_database3::<Fix>();
            f(&fix, repo);
        }
        TestType::Both => {
            println!("Starting Unit:");
            let (fix, repo) = setup_mock::<Fix>();
            f(&fix, repo);
            println!("Starting Integration:");
            let (fix, repo, _cleanup_wrapper) = setup_database3::<Fix>();
            f(&fix, repo);
        }
    }
}

fn setup_mock_impl<Fix>() -> (Fix, Arc<Mutex<MockDatabase>>)
where
    Fix: Fixture,
{
    let db = Arc::new(Mutex::new(MockDatabase::default()));
    let db_clone: BoxedRepository = Box::new(db.clone());
    let fixture = Fix::generate(&db_clone);
    (fixture, db)
}

/// Sets up a fixture and a mock repository
pub fn setup_mock<Fix>() -> (Fix, BoxedRepository)
where
    Fix: Fixture,
{
    let (fixture, db) = setup_mock_impl();
    (fixture, Box::new(db))
}

/// Sets up a provider of mocks
pub fn setup_mock_provider<Fix>() -> (Fix, RepositoryProvider)
where
    Fix: Fixture,
{
    let (fixture, db) = setup_mock_impl();
    (fixture, RepositoryProvider::Mock(db))
}

/// Sets up a fixture for a database-backed repository.
/// It will create the database from scratch before the test runs.
/// It will drop the database once the test completes.
pub fn setup_database3<Fix>() -> (Fix, BoxedRepository, Cleanup)
where
    Fix: Fixture,
{
    use diesel::Connection;
    let admin_conn = PgConnection::establish(DROP_DATABASE_URL).unwrap();
    let (pool, cleanup) = setup_pool_random_db(
        admin_conn,
        "postgres://hzimmerman:password@localhost",
        MIGRATIONS_DIRECTORY,
    );
    let conn = pool.get().unwrap();
    let conn: BoxedRepository = Box::new(conn);
    let fixture = Fix::generate(&conn);
    (fixture, conn, cleanup)
}

/// sets up a pool and executes a provided test that utilizes the pool
pub fn execute_pool_test2<Fun, Fix>(mut test_function: Fun)
where
    Fun: FnMut(&Fix, RepositoryProvider),
    Fix: Fixture,
{
    use diesel::Connection;
    let admin_conn = PgConnection::establish(DROP_DATABASE_URL).unwrap();
    let (pool, _cleanup) = setup_pool_random_db(
        admin_conn,
        "postgres://hzimmerman:password@localhost",
        MIGRATIONS_DIRECTORY,
    );
    let conn = pool.get().unwrap();
    let conn: BoxedRepository = Box::new(conn);
    let fixture = Fix::generate(&conn);

    test_function(&fixture, RepositoryProvider::Pool(pool));
}

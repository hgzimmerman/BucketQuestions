//! Test module for convienence functions and fixtures
pub mod answer_fixture;
pub mod bucket_fixture;
pub mod bucket_user_relation_fixture;
pub mod empty_fixture;
//pub mod mock;
pub mod question_fixture;
pub mod user_fixture;
pub mod fixture;

//use self::mock::MockDatabase;

use self::fixture::Fixture;
use crate::{RepositoryProvider, BoxedRepository};
use diesel::PgConnection;
use std::sync::{Mutex, Arc};
use crate::mock::MockDatabase;
use diesel_reset::setup::{setup_pool_sequential};

/// Sets up a fixture and repository to a state defined by the fixture's initialization function.
/// The repository implementation is chosen by a feature flag.
///
/// If the binary is compiled with the `integration` flag enabled, it will use the database.
/// Otherwise, it will use the mock object more suitable for unit testing.
pub fn setup<Fix>() -> (Fix, BoxedRepository)
where
    Fix: Fixture,
{
    if !cfg!(feature = "integration") {
        setup_mock()
    } else {
        setup_database()
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


/// Sets up a fixture and a database-backed repository
pub fn setup_database<Fix>() -> (Fix, BoxedRepository)
where
    Fix: Fixture,
{
    let db: PgConnection = diesel_reset::setup::setup_single_connection();
    let db: BoxedRepository = Box::new(db);
    let fixture = Fix::generate(&db);
    (fixture, db)
}


/// Sets up a single pooled connection default state.
//pub fn setup_pooled_conn<Fix>() -> (Fix, AbstractRepository)
//where
//Fix: Fixture,
//{
//    let db: Pool = diesel_reset::setup::setup_pool();
//    let con = db.get().unwrap();
//    let db: AbstractRepository = Box::new(con);
//    let fixture = Fix::generate(&db);
//    (fixture, db)
//}

////#[warn(reason =  "Does not hold lock to db mutex")]
///// Sets up a repository provider in a default state.
//pub fn setup_pool<Fix>() -> (Fix, RepositoryProvider)
//where
//Fix: Fixture,
//{
//    let pool: Pool = diesel_reset::setup::setup_pool();
//    let con= pool.get().unwrap();
//    let db: AbstractRepository = Box::new(con);
//    let fixture = Fix::generate(&db);
//    (fixture, RepositoryProvider::Pool(pool))
//}


/// sets up a pool and executes a provided test that utilizes the pool
pub fn execute_pool_test<Fun, Fix>(mut test_function: Fun)
where
    Fun: FnMut(&Fix, RepositoryProvider),
    Fix: Fixture,
{
    // The lock is dropped at the end of this scope, preventing other tests from running until then.
    let (pool, _lock) = setup_pool_sequential();
    let conn = pool.get().unwrap();
    let conn: BoxedRepository = Box::new(conn);
    let fixture = Fix::generate(&conn);

    test_function(&fixture, RepositoryProvider::Pool(pool));
}
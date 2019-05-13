//! Test module for convienence functions and fixtures
pub mod answer_fixture;
pub mod bucket_fixture;
pub mod bucket_user_relation_fixture;
pub mod empty_fixture;
//pub mod mock;
pub mod question_fixture;
pub mod user_fixture;

//use self::mock::MockDatabase;

use crate::{Repository, RepositoryProvider, AbstractRepository};
use diesel::PgConnection;
use diesel_reset::fixture::Fixture;
use std::sync::{Mutex, Arc};
use pool::{Pool, PooledConn};
use crate::mock::MockDatabase;

/// Sets up a fixture and repository to a state defined by the fixture's initialization function.
/// The repository implementation is chosen by a feature flag.
///
/// If the binary is compiled with the `integration` flag enabled, it will use the database.
/// Otherwise, it will use the mock object more suitable for unit testing.
pub fn setup<Fix>() -> (Fix, AbstractRepository)
where
    Fix: Fixture<Repository = AbstractRepository>,
{
    if !cfg!(feature = "integration") {
        setup_mock()
    } else {
        setup_database()
    }
}

fn setup_mock_impl<Fix>() -> (Fix, Arc<Mutex<MockDatabase>>)
where
    Fix: Fixture<Repository = AbstractRepository>,
{
    let db = Arc::new(Mutex::new(MockDatabase::default()));
    let db_clone: AbstractRepository = Box::new(db.clone());
    let fixture = Fix::generate(&db_clone);
    (fixture, db)
}

/// Sets up a fixture and a mock repository
pub fn setup_mock<Fix>() -> (Fix, AbstractRepository)
where
    Fix: Fixture<Repository = AbstractRepository>,
{
    let (fixture, db) = setup_mock_impl();
    (fixture, Box::new(db))
}

/// Sets up a provider of mocks
pub fn setup_mock_provider<Fix>() -> (Fix, RepositoryProvider)
where
    Fix: Fixture<Repository = AbstractRepository>,
{
    let (fixture, db) = setup_mock_impl();
    (fixture, RepositoryProvider::Mock(db))
}

/// Sets up a fixture and a database-backed repository
pub fn setup_database<Fix>() -> (Fix, AbstractRepository)
where
    Fix: Fixture<Repository = AbstractRepository>,
{
    let db: PgConnection = diesel_reset::setup::setup_single_connection();
    let db: AbstractRepository = Box::new(db);
    let fixture = Fix::generate(&db);
    (fixture, db)
}


/// Sets up a single pooled connection default state.
pub fn setup_pooled_conn<Fix>() -> (Fix, AbstractRepository)
where
Fix: Fixture<Repository = AbstractRepository>,
{
    let db: Pool = diesel_reset::setup::setup_pool();
    let con = db.get().unwrap();
    let db: AbstractRepository = Box::new(con);
    let fixture = Fix::generate(&db);
    (fixture, db)
}

//#[warn(reason =  "Does not hold lock to db mutex")]
/// Sets up a repository provider in a default state.
pub fn setup_pool<Fix>() -> (Fix, RepositoryProvider)
where
Fix: Fixture<Repository = AbstractRepository>,
{
    let pool: Pool = diesel_reset::setup::setup_pool();
    let con= pool.get().unwrap();
    let db: AbstractRepository = Box::new(con);
    let fixture = Fix::generate(&db);
    (fixture, RepositoryProvider::Pool(pool))
}

//pub fn setup_pool_2<Fun, Fix>(mut test_function: Fun)
//where
//    Fun: FnMut(&Fix, AbstractRepository),
//    Fix: Fixture<Repository=AbstractRepository>,
//{
//    diesel_reset::setup::setup_pool2(test_function)
//}


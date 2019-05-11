//! Test module for convienence functions and fixtures
pub mod answer_fixture;
pub mod bucket_fixture;
pub mod bucket_user_relation_fixture;
pub mod empty_fixture;
pub mod mock;
pub mod question_fixture;
pub mod user_fixture;

use self::mock::MockDatabase;
use crate::{Repository, RepoProvider};
use diesel::PgConnection;
use diesel_reset::fixture::Fixture;
use std::sync::{Mutex, Arc};
use pool::{Pool, PooledConn};

/// Sets up a fixture and repository to a state defined by the fixture's initialization function.
/// The repository implementation is chosen by a feature flag.
///
/// If the binary is compiled with the `integration` flag enabled, it will use the database.
/// Otherwise, it will use the mock object more suitable for unit testing.
pub fn setup<Fix>() -> (Fix, Box<Repository>)
where
    Fix: Fixture<Repository = Box<Repository>>,
{
    if !cfg!(feature = "integration") {
        setup_mock()
    } else {
        setup_database()
    }
}

/// Sets up a fixture and a mock repository
pub fn setup_mock<Fix>() -> (Fix, Box<Repository>)
where
    Fix: Fixture<Repository = Box<Repository>>,
{
    let db = Arc::new(Mutex::new(MockDatabase::default()));
    let db: Box<dyn Repository> = Box::new(db);
    let fixture = Fix::generate(&db);
    (fixture, db)
}

/// Sets up a fixture and a database-backed repository
pub fn setup_database<Fix>() -> (Fix, Box<Repository>)
where
    Fix: Fixture<Repository = Box<Repository>>,
{
    let db: PgConnection = diesel_reset::setup::setup_single_connection();
    let db: Box<dyn Repository> = Box::new(db);
    let fixture = Fix::generate(&db);
    (fixture, db)
}


pub fn setup_pooled_conn<Fix>() -> (Fix, Box<Repository>)
where
Fix: Fixture<Repository = Box<Repository>>,
{
    let db: Pool = diesel_reset::setup::setup_pool();
    let con: PooledConn = db.get().unwrap();
    let db: Box<dyn Repository> = Box::new(con);
    let fixture = Fix::generate(&db);
    (fixture, db)
}


pub fn setup_pool<Fix>() -> (Fix, Box<RepoProvider>)
where
Fix: Fixture<Repository = Box<Repository>>,
{
    let pool: Pool = diesel_reset::setup::setup_pool();
    let con: PooledConn = pool.get().unwrap();
    let db: Box<dyn Repository> = Box::new(con);
    let fixture = Fix::generate(&db);
    (fixture, Box::new(pool))
}


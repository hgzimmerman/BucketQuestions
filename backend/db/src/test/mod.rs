//! Test module for convienence functions and fixtures
pub mod user_fixture;
pub mod empty_fixture;
pub mod bucket_fixture;
pub mod bucket_user_relation_fixture;
pub mod question_fixture;


use crate::Repository;
use diesel_reset::fixture::Fixture;
use std::sync::Mutex;
use diesel::PgConnection;
use crate::mock::MockDatabase;

/// Sets up a fixture and repository to a state defined by the fixture's initialization function.
/// The repository implementation is chosen by a feature flag.
///
/// If the binary is compiled with the `integration` flag enabled, it will use the database.
/// Otherwise, it will use the mock object more suitable for unit testing.
pub fn setup<Fix>() -> (Fix,  Box<Repository>)
where
    Fix: Fixture<Repository = Box<Repository>>
{
    if !cfg!(feature="integration") {
        setup_mock()
    } else {
        setup_database()
    }
}


/// Sets up a fixture and a mock repository
pub fn setup_mock<Fix>() -> (Fix, Box<Repository>)
where
    Fix: Fixture<Repository = Box<Repository>>
{
    let db = Mutex::new(MockDatabase::default());
    let db: Box<dyn Repository> = Box::new(db);
    let fixture = Fix::generate(&db);
    (fixture, db)
}

/// Sets up a fixture and a database-backed repository
pub fn setup_database<Fix>() -> (Fix, Box<Repository>)
where
    Fix: Fixture<Repository = Box<Repository>>
{
    let db: PgConnection = diesel_reset::setup::setup_single_connection();
    let db: Box<dyn Repository> = Box::new(db);
    let fixture = Fix::generate(&db);
    (fixture, db)
}


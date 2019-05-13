//! Fixture abstraction.

use crate::AbstractRepository;

/// The Fixture trait should be implemented for collections of data used in testing.
/// Because it can be instantiated using just a connection to the database,
/// it allows the creation of the type in question and allows data generated at row insertion time
/// (UUIDs) to be made available to the body of tests.
///
/// The overall architecture of the fixture system means that each test starts with a virgin database,
/// which a given implementor of fixture is responsible for populating to a defined state.
/// The test is then executed, and the database is then tore down.
pub trait Fixture {
    /// Takes the abstract repository and sets it up and stored details about the setup in the returned fixture.
    fn generate(repository: &AbstractRepository) -> Self;
}


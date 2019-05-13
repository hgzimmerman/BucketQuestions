//! Fixture abstraction.

/// The Fixture trait should be implemented for collections of data used in testing.
/// Because it can be instantiated using just a connection to the database,
/// it allows the creation of the type in question and allows data generated at row insertion time
/// (UUIDs) to be made available to the body of tests.
///
/// The overall architecture of the fixture system means that each test starts with a virgin database,
/// which a given implementor of fixture is responsible for populating to a defined state.
/// The test is then executed, and the database is then tore down.
pub trait Fixture {
    type Repository; // TODO Fixture trait might need to move into db crate. Get rid of this type constraint, and make the fixture trait work directly with AbstractRepository.
    fn generate(repository: &Self::Repository) -> Self;
}


///// Because some tests may not require any initial database state, but still utilize the connection,
///// This Fixture is provided to meet that need.
//pub struct EmptyFixture;
//
//impl Fixture for EmptyFixture {
//    type Repository = ();
//    fn generate(repository: &Self::Repository) -> Self {
//        EmptyFixture
//    }
//}


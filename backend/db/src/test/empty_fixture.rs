use crate::Repository;
use diesel_reset::fixture::Fixture;

/// Empty fixture that makes no changes to the repository.
pub struct EmptyFixture;

impl Fixture for EmptyFixture {
    type Repository = Box<dyn Repository>;
    fn generate(_: &Box<Repository>) -> Self  {
        EmptyFixture
    }
}
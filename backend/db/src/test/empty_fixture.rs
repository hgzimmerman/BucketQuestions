use crate::Repository;
use diesel_reset::fixture::Fixture;

/// Empty fixture that makes no changes to the repository.
#[derive(Clone, Copy, Debug)]
pub struct EmptyFixture;

impl Fixture for EmptyFixture {
    type Repository = Box<dyn Repository>;
    fn generate(_: &Box<Repository>) -> Self {
        EmptyFixture
    }
}

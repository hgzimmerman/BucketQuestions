//! A fixture for testing against empty repository configurations.
use crate::{Repository, AbstractRepository};
use diesel_reset::fixture::Fixture;

/// Empty fixture that makes no changes to the repository.
#[derive(Clone, Copy, Debug)]
pub struct EmptyFixture;

impl Fixture for EmptyFixture {
    type Repository = AbstractRepository;
    fn generate(_: &AbstractRepository) -> Self {
        EmptyFixture
    }
}

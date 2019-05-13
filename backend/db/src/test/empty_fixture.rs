//! A fixture for testing against empty repository configurations.
use crate::{Repository, AbstractRepository};
use crate::test::fixture::Fixture;

/// Empty fixture that makes no changes to the repository.
#[derive(Clone, Copy, Debug)]
pub struct EmptyFixture;

impl Fixture for EmptyFixture {
    fn generate(_: &AbstractRepository) -> Self {
        EmptyFixture
    }
}

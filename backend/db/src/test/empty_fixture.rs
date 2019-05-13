//! A fixture for testing against empty repository configurations.
use crate::{BoxedRepository};
use crate::test::fixture::Fixture;

/// Empty fixture that makes no changes to the repository.
#[derive(Clone, Copy, Debug)]
pub struct EmptyFixture;

impl Fixture for EmptyFixture {
    fn generate(_: &BoxedRepository) -> Self {
        EmptyFixture
    }
}

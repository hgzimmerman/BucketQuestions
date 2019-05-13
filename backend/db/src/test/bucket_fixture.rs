//! A fixture for testing against configurations related to buckets.
use crate::{bucket::db_types::{Bucket, NewBucket}, BoxedRepository};
use crate::test::fixture::Fixture;

/// Fixture that creates one user record in the repository.
#[derive(Clone, Debug)]
pub struct BucketFixture {
    /// Becket
    pub bucket: Bucket,
}

impl Fixture for BucketFixture {

    fn generate(conn: &BoxedRepository) -> Self {
        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string(),
        };
        let bucket = conn
            .create_bucket(new_bucket)
            .expect("Should be able to create bucket");
        BucketFixture { bucket }
    }
}

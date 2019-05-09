use crate::Repository;
use diesel_reset::fixture::Fixture;
use crate::bucket::db_types::{NewBucket, Bucket};

/// Fixture that creates one user record in the repository.
pub struct BucketFixture {
    pub bucket: Bucket
}

impl Fixture for BucketFixture
{
    type Repository = Box<dyn Repository>;

    fn generate(conn: &Box<Repository>) -> Self  {
        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string()
        };
        let bucket = conn.create_bucket(new_bucket).expect("Should be able to create bucket");
        BucketFixture { bucket }
    }
}
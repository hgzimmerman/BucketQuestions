//! A fixture for testing against configurations related to bucket user relations, but without the relation already existing.
use crate::{bucket::db_types::{Bucket, BucketUserRelation, NewBucket, NewBucketUserRelation}, user::db_types::{NewUser, User}, BoxedRepository};
use crate::test::user_fixture::UserFixture;
use crate::test::fixture::Fixture;

/// Fixture that creates 2 users, and 1 bucket.
/// user1 is joined to the bucket.
#[derive(Clone, Debug)]
pub struct BucketAndUserFixture {
    /// Bucket
    pub bucket: Bucket,
    /// First user
    pub user1: User,
    /// Second user
    pub user2: User,
}

impl Fixture for BucketAndUserFixture {
    fn generate(conn: &BoxedRepository) -> Self {
        let user1 = UserFixture::generate(conn).user;

        let new_user_2 = NewUser {
            google_user_id: "987654321".to_string(),
            google_name: Some("Yote".to_owned()),
        };

        let user2 = conn.create_user(new_user_2).unwrap();

        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string(),
        };
        let bucket = conn
            .create_bucket(new_bucket)
            .expect("Should be able to create bucket");


        BucketAndUserFixture {
            bucket,
            user1,
            user2,
        }
    }
}
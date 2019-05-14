//! A fixture for testing against configurations related to bucket user relations.
use crate::{bucket::db_types::{Bucket, BucketUserRelation, NewBucket, NewBucketUserRelation}, user::db_types::{NewUser, User}, BoxedRepository};
use crate::test::user_fixture::UserFixture;
use crate::test::fixture::Fixture;

/// Fixture that creates 2 users, 1 bucket, and one relation record in the repository.
/// user1 is joined to the bucket.
#[derive(Clone, Debug)]
pub struct UserBucketRelationFixture {
    /// Bucket
    pub bucket: Bucket,
    /// First user
    pub user1: User,
    /// Second user
    pub user2: User,
    /// Relation between first user and bucket
    pub relation: BucketUserRelation,
}

impl Fixture for UserBucketRelationFixture {
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

        let new_relation = NewBucketUserRelation {
            user_uuid: user1.uuid,
            bucket_uuid: bucket.uuid,
            set_public_permission: true,
            set_drawing_permission: true,
            set_exclusive_permission: true,
            grant_permissions_permission: true,
        };

        let relation = conn
            .add_user_to_bucket(new_relation)
            .expect("Should be able to create user bucket relation");

        UserBucketRelationFixture {
            bucket,
            user1,
            user2,
            relation,
        }
    }
}

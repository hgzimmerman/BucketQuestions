use crate::Repository;
use diesel_reset::fixture::Fixture;
use crate::bucket::db_types::{NewBucket, Bucket, BucketUserRelation, NewBucketUserRelation};
use crate::user::{NewUser, User};

/// Fixture that creates 2 users, 1 bucket, and one relation record in the repository.
/// user1 is joined to the bucket.
pub struct UserBucketRelationFixture {
    pub bucket: Bucket,
    pub user1: User,
    pub user2: User,
    pub relation: BucketUserRelation
}

impl Fixture for UserBucketRelationFixture
{
    type Repository = Box<dyn Repository>;

    fn generate(conn: &Box<Repository>) -> Self  {

        let new_user_1 = NewUser {
            google_user_id: "123456789".to_string(),
            google_name: Some("Yeet".to_owned())
        };

        let user1 = conn.create_user(new_user_1).unwrap();

        let new_user_2 = NewUser {
            google_user_id: "987654321".to_string(),
            google_name: Some("Yote".to_owned())
        };

        let user2 = conn.create_user(new_user_2).unwrap();


        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string()
        };
        let bucket = conn.create_bucket(new_bucket).expect("Should be able to create bucket");

        let new_relation = NewBucketUserRelation {
            user_uuid: user1.uuid,
            bucket_uuid: bucket.uuid,
            set_public_permission: true,
            set_drawing_permission: true,
            set_exclusive_permission: true,
            grant_permissions_permission: true
        };

        let relation = conn.add_user_to_bucket(new_relation).expect("Should be able to create user bucket relation");

        UserBucketRelationFixture {
            bucket,
            user1,
            user2,
            relation,
        }
    }
}
use crate::test::empty_fixture::EmptyFixture;
use crate::test::setup;
use crate::bucket::db_types::{NewBucket, BucketFlagChangeset};
use crate::test::bucket_fixture::BucketFixture;
use diesel::result::Error;

mod bucket {
    use super::*;
    #[test]
    fn create_bucket() {
        let (_fixture, db) = setup::<EmptyFixture>();

        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string()
        };
        db.create_bucket(new_bucket).expect("Bucket should be created");
    }

    #[test]
    fn create_bucket_default_flags() {
        let (_fixture, db) = setup::<EmptyFixture>();

        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string()
        };
        let bucket = db.create_bucket(new_bucket).expect("Bucket should be created");
        assert!(bucket.public_viewable);
        assert!(bucket.drawing_enabled);
        assert!(!bucket.exclusive);
    }

    #[test]
    fn get_bucket_uuid() {
        let (fixture, db) = setup::<BucketFixture>();
        assert_eq!(db.get_bucket_by_uuid(fixture.bucket.uuid), Ok(fixture.bucket));
    }

    #[test]
    fn get_bucket_slug() {
        let (fixture, db) = setup::<BucketFixture>();
        assert_eq!(db.get_bucket_by_slug(fixture.bucket.bucket_slug.clone()), Ok(fixture.bucket));
    }

    #[test]
    fn delete_bucket() {
        let (fixture, db) = setup::<BucketFixture>();
        db.delete_bucket(fixture.bucket.uuid).expect("Should delete bucket");
        assert_eq!(db.get_bucket_by_uuid(fixture.bucket.uuid), Err(Error::NotFound));
    }


    #[test]
    fn change_visibility_bucket() {
        let (fixture, db) = setup::<BucketFixture>();
        let mut changeset = BucketFlagChangeset {
            uuid: fixture.bucket.uuid,
            public_viewable: Some(true),
            drawing_enabled: None,
            exclusive: None
        };
        let bucket = db.change_bucket_flags(changeset).expect("Should be able to change visibility");
        assert!(bucket.public_viewable);

        changeset.public_viewable = Some(false); // set to false
        let bucket = db.change_bucket_flags(changeset).expect("Should be able to change visibility");
        assert!(!bucket.public_viewable);
    }

    #[test]
    fn bucket_all_none_changeset_does_not_affect_record() {
        let (fixture, db) = setup::<BucketFixture>();
        let changeset = BucketFlagChangeset {
            uuid: fixture.bucket.uuid,
            public_viewable: None,
            drawing_enabled: None,
            exclusive: None
        };
        let bucket = db.change_bucket_flags(changeset).expect("Should be able to send an empty changeset");
        assert_eq!(bucket, fixture.bucket)
    }

    #[test]
    fn get_visible_buckets() {
        let (fixture, db) = setup::<BucketFixture>();
        let changeset = BucketFlagChangeset {
            uuid: fixture.bucket.uuid,
            public_viewable: Some(true),
            drawing_enabled: None,
            exclusive: None
        };
        let _bucket = db.change_bucket_flags(changeset).expect("Should be able to change visibility");

        let visible_buckets = db.get_publicly_visible_buckets().expect("Should find public buckets");
        assert!(visible_buckets.contains(&fixture.bucket))
    }
}

mod bucket_user_relation {
    use super::*;
    use crate::test::bucket_user_relation_fixture::UserBucketRelationFixture;
    use crate::bucket::db_types::{NewBucketUserRelation, BucketUserPermissionsChangeset};

    #[test]
    fn create_relation() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let new_relation = NewBucketUserRelation {
            user_uuid: fixture.user2.uuid,
            bucket_uuid: fixture.bucket.uuid,
            set_public_permission: false,
            set_drawing_permission: false,
            set_exclusive_permission: false,
            grant_permissions_permission: false
        };
        db.add_user_to_bucket(new_relation).expect("Should be able to add user to bucket");
    }

    #[test]
    fn cant_create_duplicate_relation() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let new_relation = NewBucketUserRelation {
            user_uuid: fixture.user1.uuid, // User 1 already has a join.
            bucket_uuid: fixture.bucket.uuid,
            set_public_permission: false,
            set_drawing_permission: false,
            set_exclusive_permission: false,
            grant_permissions_permission: false
        };
        db.add_user_to_bucket(new_relation).expect_err("Should not able to add user to bucket twice");
    }

    #[test]
    fn remove_user_from_bucket() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let relation = db.remove_user_from_bucket(fixture.user1.uuid, fixture.bucket.uuid).expect("Should be able to remove user");
        assert_eq!(relation, fixture.relation);
        db.get_user_bucket_relation(fixture.user1.uuid, fixture.bucket.uuid).expect_err("Relation should be deleted");
    }

    #[test]
    fn cant_remove_unrelated_user_from_bucket() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let _relation = db.remove_user_from_bucket(fixture.user2.uuid, fixture.bucket.uuid).expect_err("Should not able to remove user not in bucket");
    }

    #[test]
    fn set_permissions() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let changeset = BucketUserPermissionsChangeset {
            user_uuid: fixture.user1.uuid,
            bucket_uuid: fixture.bucket.uuid,
            set_public_permission: None,
            set_drawing_permission: None,
            set_exclusive_permission: None,
            grant_permissions_permission: Some(false)
        };

        assert_eq!(fixture.relation.grant_permissions_permission, true); // precondition

        let relation = db.set_permissions(changeset).expect("Should be able to set permissions");
        assert_eq!(relation.grant_permissions_permission, false);
    }

    #[test]
    fn set_empty_permissions() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let changeset = BucketUserPermissionsChangeset {
            user_uuid: fixture.user1.uuid,
            bucket_uuid: fixture.bucket.uuid,
            set_public_permission: None,
            set_drawing_permission: None,
            set_exclusive_permission: None,
            grant_permissions_permission: None
        };
        let _relation = db.set_permissions(changeset).expect("Should be able to set empty permissions");
    }

    #[test]
    fn get_relation() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let relation = db.get_user_bucket_relation(fixture.user1.uuid, fixture.bucket.uuid).expect("Should get relation");
        assert_eq!(relation, fixture.relation);
    }

    #[test]
    fn cant_get_relation() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let _relation = db.get_user_bucket_relation(fixture.user2.uuid, fixture.bucket.uuid).expect_err("Should not get relation");
    }

    #[test]
    fn get_associated_users() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let users = db.get_users_in_bucket(fixture.bucket.uuid).expect("Should get users");
        assert_eq!(users.len(), 1);
        assert_eq!(users.get(0).expect("Should get user"), &fixture.user1);
    }

    #[test]
    fn get_associated_buckets() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let users = db.get_buckets_user_is_a_part_of(fixture.user1.uuid).expect("Should get related buckets");
        assert_eq!(users.len(), 1);
        assert_eq!(users.get(0).unwrap(), &fixture.bucket);
    }

    #[test]
    fn dont_get_unassociated_buckets() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();
        let users = db.get_buckets_user_is_a_part_of(fixture.user2.uuid).expect("Should get related buckets");
        assert_eq!(users.len(), 0);
    }
}

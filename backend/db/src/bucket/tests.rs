use crate::{
    bucket::db_types::{BucketFlagChangeset, NewBucket},
    test::{bucket_fixture::BucketFixture, empty_fixture::EmptyFixture},
};
use diesel::result::Error;

mod bucket {
    use super::*;
    use crate::{test::util::execute_test, BoxedRepository};

    #[test]
    fn create_bucket() {
        execute_test(|_fixture: &EmptyFixture, db: BoxedRepository| {
            let new_bucket = NewBucket {
                bucket_name: "bucket".to_string(),
                bucket_slug: "slug".to_string(),
            };
            db.create_bucket(new_bucket)
                .expect("Bucket should be created");
        })
    }

    #[test]
    fn create_bucket_default_flags() {
        execute_test(|_fixture: &EmptyFixture, db: BoxedRepository| {
            let new_bucket = NewBucket {
                bucket_name: "bucket".to_string(),
                bucket_slug: "slug".to_string(),
            };
            let bucket = db
                .create_bucket(new_bucket)
                .expect("Bucket should be created");
            assert!(bucket.public_viewable);
            assert!(bucket.drawing_enabled);
            assert!(!bucket.exclusive);
        });
    }

    #[test]
    fn get_bucket_uuid() {
        execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
            assert_eq!(
                db.get_bucket_by_uuid(fixture.bucket.uuid),
                Ok(fixture.bucket.clone())
            );
        });
    }

    #[test]
    fn get_bucket_slug() {
        execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
            assert_eq!(
                db.get_bucket_by_slug(fixture.bucket.bucket_slug.clone()),
                Ok(fixture.bucket.clone())
            );
        });
    }

    #[test]
    fn delete_bucket() {
        execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
            db.delete_bucket(fixture.bucket.uuid)
                .expect("Should delete bucket");
            assert_eq!(
                db.get_bucket_by_uuid(fixture.bucket.uuid),
                Err(Error::NotFound)
            );
        });
    }

    #[test]
    fn change_visibility_bucket() {
        execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
            let mut changeset = BucketFlagChangeset {
                uuid: fixture.bucket.uuid,
                public_viewable: Some(true),
                drawing_enabled: None,
                exclusive: None,
            };
            let bucket = db
                .change_bucket_flags(changeset)
                .expect("Should be able to change visibility");
            assert!(bucket.public_viewable);

            changeset.public_viewable = Some(false); // set to false
            let bucket = db
                .change_bucket_flags(changeset)
                .expect("Should be able to change visibility");
            assert!(!bucket.public_viewable);
        });
    }

    #[test]
    fn bucket_all_none_changeset_does_not_affect_record() {
        execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
            let changeset = BucketFlagChangeset {
                uuid: fixture.bucket.uuid,
                public_viewable: None,
                drawing_enabled: None,
                exclusive: None,
            };
            let bucket = db
                .change_bucket_flags(changeset)
                .expect("Should be able to send an empty changeset");
            assert_eq!(bucket, fixture.bucket)
        });
    }

    #[test]
    fn get_visible_buckets() {
        execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
            let changeset = BucketFlagChangeset {
                uuid: fixture.bucket.uuid,
                public_viewable: Some(true),
                drawing_enabled: None,
                exclusive: None,
            };
            let _bucket = db
                .change_bucket_flags(changeset)
                .expect("Should be able to change visibility");

            let visible_buckets = db
                .get_publicly_visible_buckets()
                .expect("Should find public buckets");
            assert!(visible_buckets.contains(&fixture.bucket))
        });
    }
}

use crate::{
    bucket::db_types::{BucketFlagChangeset, NewBucket},
    test::{bucket_fixture::BucketFixture, empty_fixture::EmptyFixture, setup},
};
use diesel::result::Error;

mod bucket {
    use super::*;
    #[test]
    fn create_bucket() {
        let (_fixture, db) = setup::<EmptyFixture>();

        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string(),
        };
        db.create_bucket(new_bucket)
            .expect("Bucket should be created");
    }

    #[test]
    fn create_bucket_default_flags() {
        let (_fixture, db) = setup::<EmptyFixture>();

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
    }

    #[test]
    fn get_bucket_uuid() {
        let (fixture, db) = setup::<BucketFixture>();
        assert_eq!(
            db.get_bucket_by_uuid(fixture.bucket.uuid),
            Ok(fixture.bucket)
        );
    }

    #[test]
    fn get_bucket_slug() {
        let (fixture, db) = setup::<BucketFixture>();
        assert_eq!(
            db.get_bucket_by_slug(fixture.bucket.bucket_slug.clone()),
            Ok(fixture.bucket)
        );
    }

    #[test]
    fn delete_bucket() {
        let (fixture, db) = setup::<BucketFixture>();
        db.delete_bucket(fixture.bucket.uuid)
            .expect("Should delete bucket");
        assert_eq!(
            db.get_bucket_by_uuid(fixture.bucket.uuid),
            Err(Error::NotFound)
        );
    }

    #[test]
    fn change_visibility_bucket() {
        let (fixture, db) = setup::<BucketFixture>();
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
    }

    #[test]
    fn bucket_all_none_changeset_does_not_affect_record() {
        let (fixture, db) = setup::<BucketFixture>();
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
    }

    #[test]
    fn get_visible_buckets() {
        let (fixture, db) = setup::<BucketFixture>();
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
    }
}

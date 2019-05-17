//! Mock impl
use crate::{
    bucket::{
        db_types::{Bucket, BucketFlagChangeset, NewBucket},
        interface::BucketRepository,
    },
    fake::{DummyDbErrorInfo, FakeDatabase},
};
use diesel::result::{DatabaseErrorKind, Error};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

impl BucketRepository for Arc<Mutex<FakeDatabase>> {
    fn create_bucket(&self, new_bucket: NewBucket) -> Result<Bucket, Error> {
        let mut db = self.lock().unwrap();
        let uuid = Uuid::new_v4();
        let bucket = Bucket {
            uuid,
            bucket_name: new_bucket.bucket_name,
            bucket_slug: new_bucket.bucket_slug,
            public_viewable: true,
            drawing_enabled: true,
            exclusive: false,
            updated_at: chrono::Utc::now().naive_utc(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        if db.buckets.iter().find(|b| b.uuid == uuid).is_some() {
            return Err(Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                Box::new(DummyDbErrorInfo::new()),
            ));
        }
        db.buckets.push(bucket.clone());
        return Ok(bucket);
    }

    fn delete_bucket(&self, bucket_uuid: Uuid) -> Result<Bucket, Error> {
        let mut db = self.lock().unwrap();
        let index = db
            .buckets
            .iter()
            .position(|b| b.uuid == bucket_uuid)
            .ok_or_else(|| Error::NotFound)?;
        Ok(db.buckets.remove(index))
    }

    fn get_publicly_visible_buckets(&self) -> Result<Vec<Bucket>, Error> {
        let db = self.lock().unwrap();
        let visible = db
            .buckets
            .iter()
            .filter(|b| b.public_viewable)
            .cloned()
            .collect();
        Ok(visible)
    }

    fn get_bucket_by_slug(&self, slug: String) -> Result<Bucket, Error> {
        let db = self.lock().unwrap();
        db.buckets
            .iter()
            .find(|b| b.bucket_slug == slug)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }

    fn get_bucket_by_uuid(&self, uuid: Uuid) -> Result<Bucket, Error> {
        let db = self.lock().unwrap();
        db.buckets
            .iter()
            .find(|b| b.uuid == uuid)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }

    fn change_bucket_flags(&self, changeset: BucketFlagChangeset) -> Result<Bucket, Error> {
        let mut db = self.lock().unwrap();
        let uuid = changeset.uuid;
        let bucket = db
            .buckets
            .iter_mut()
            .find(|b| b.uuid == uuid)
            .ok_or_else(|| Error::NotFound)?;

        if let Some(visible) = changeset.public_viewable {
            bucket.public_viewable = visible;
        }
        if let Some(drawing_enabled) = changeset.drawing_enabled {
            bucket.drawing_enabled = drawing_enabled;
        }
        if let Some(private) = changeset.exclusive {
            bucket.exclusive = private;
        }

        Ok(bucket.clone())
    }
}

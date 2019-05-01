//! Implementation of the specified interfaces for PgConnection.

use crate::bucket::interface::BucketRepository;
use diesel::pg::PgConnection;
use crate::bucket::db_types::{NewBucket, Bucket};
use diesel::result::Error;
use uuid::Uuid;
use crate::schema::buckets;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;


impl BucketRepository for PgConnection {
    fn create_bucket(&self, new_bucket: NewBucket) -> Result<Bucket, Error> {
        crate::util::create_row(buckets::table, new_bucket, self)
    }

    fn delete_bucket(&self, bucket_uuid: Uuid) -> Result<Bucket, Error> {
        crate::util::delete_row(buckets::table, bucket_uuid, self)
    }

    fn get_publicly_visible_buckets(&self) -> Result<Vec<Bucket>, Error> {
        buckets::table
            .filter(buckets::visible.eq(true))
            .get_results(self)
    }

    fn get_bucket_by_slug(&self, slug: String) -> Result<Bucket, Error> {
        buckets::table
            .filter(&buckets::bucket_slug.eq(slug))
            .first(self)
    }

    fn get_bucket_by_uuid(&self, uuid: Uuid) -> Result<Bucket, Error> {
        crate::util::get_row(buckets::table, uuid, self)
    }

    fn change_visibility(&self, bucket_uuid: Uuid, visible: bool) -> Result<Bucket, Error> {
        let target = buckets::table
            .find(bucket_uuid);

        diesel::update(target)
            .set(buckets::visible.eq(visible))
            .get_result(self)
    }

    fn change_drawing_status(&self, bucket_uuid: Uuid, drawing: bool) -> Result<Bucket, Error> {
        let target = buckets::table
            .find(bucket_uuid);

        diesel::update(target)
            .set(buckets::drawing_enabled.eq(drawing))
            .get_result(self)
    }
}
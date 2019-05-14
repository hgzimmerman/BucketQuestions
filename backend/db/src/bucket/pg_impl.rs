//! Implementation of the specified interfaces for PgConnection.

use crate::{
    bucket::{
        db_types::{Bucket, BucketFlagChangeset, NewBucket},
        interface::BucketRepository,
    },
    schema::bucket,
    AsConnRef,
};
use diesel::{
    query_dsl::{QueryDsl, RunQueryDsl},
    result::Error,
    ExpressionMethods, SaveChangesDsl,
};
//use log::info;
use uuid::Uuid;

impl<T> BucketRepository for T
where
    T: AsConnRef,
{
    fn create_bucket(&self, new_bucket: NewBucket) -> Result<Bucket, Error> {
        crate::util::create_row(bucket::table, new_bucket, self.as_conn())
    }

    fn delete_bucket(&self, bucket_uuid: Uuid) -> Result<Bucket, Error> {
        crate::util::delete_row(bucket::table, bucket_uuid, self.as_conn())
    }

    fn get_publicly_visible_buckets(&self) -> Result<Vec<Bucket>, Error> {
        bucket::table
            .filter(bucket::public_viewable.eq(true))
            .order_by(bucket::updated_at)
            .get_results(self.as_conn())
    }

    fn get_bucket_by_slug(&self, slug: String) -> Result<Bucket, Error> {
        bucket::table
            .filter(&bucket::bucket_slug.eq(slug))
            .first(self.as_conn())
    }

    fn get_bucket_by_uuid(&self, uuid: Uuid) -> Result<Bucket, Error> {
        crate::util::get_row(bucket::table, uuid, self.as_conn())
    }

    fn change_bucket_flags(&self, changeset: BucketFlagChangeset) -> Result<Bucket, Error> {
        changeset
            .save_changes(self.as_conn())
            .or_else(|error: Error| {
                // The query will return an error if there are no changes,
                // if that is the case, just fetch the whole bucket.
                match error {
                    Error::QueryBuilderError(_) => self.get_bucket_by_uuid(changeset.uuid),
                    other => Err(other),
                }
            })
    }
}

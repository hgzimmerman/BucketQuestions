//! Specification of what functions are supported for storing data for buckets.
//!
//! These traits should try to not include significant quantities of business logic.
//! It should try to deal with only the types specified in db_types, and avoid wire types.

use crate::{
    bucket::db_types::{
        Bucket, BucketFlagChangeset,
        NewBucket,
    },
    question::db_types::Question,
    user::db_types::User,
};
use diesel::QueryResult;
use uuid::Uuid;

/// Functions for specifically working with buckets
pub trait BucketRepository {
    /// Create a bucket.
    fn create_bucket(&self, new_bucket: NewBucket) -> QueryResult<Bucket>;
    /// Delete a bucket.
    fn delete_bucket(&self, bucket_uuid: Uuid) -> QueryResult<Bucket>;
    /// Gets all publicly visible buckets.
    fn get_publicly_visible_buckets(&self) -> QueryResult<Vec<Bucket>>;
    /// Gets the bucket via its slug.
    fn get_bucket_by_slug(&self, slug: String) -> QueryResult<Bucket>;
    /// Gets the bucket via its uuid.
    fn get_bucket_by_uuid(&self, uuid: Uuid) -> QueryResult<Bucket>;
    /// Change the blags that govern the buckets behavior
    fn change_bucket_flags(&self, changeset: BucketFlagChangeset) -> QueryResult<Bucket>;
}



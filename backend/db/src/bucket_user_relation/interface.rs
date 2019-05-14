//! Specification of functions.

use crate::bucket_user_relation::db_types::{NewBucketUserRelation, BucketUserRelation, BucketUserPermissionsChangeset, BucketUserPermissions};
use diesel::QueryResult;
use uuid::Uuid;
use crate::bucket::db_types::Bucket;
use crate::user::db_types::User;

/// Functions for specifically working with bucket user relations.
pub trait BucketUserRelationRepository {
    /// Adds a user to the bucket.
    fn add_user_to_bucket(
        &self,
        relation: NewBucketUserRelation,
    ) -> QueryResult<BucketUserRelation>;
    /// Removes the user from bucket.
    fn remove_user_from_bucket(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> QueryResult<BucketUserRelation>;
    /// Get the bucket user relation
    fn get_user_bucket_relation(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> QueryResult<BucketUserRelation>;
    /// Set permissions for the user-bucket relation.
    fn set_permissions(
        &self,
        permissions_changeset: BucketUserPermissionsChangeset,
    ) -> QueryResult<BucketUserRelation>;
    /// Get the permissions for the user.
    /// The user may not be a part of the bucket.
    fn get_permissions(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> QueryResult<BucketUserPermissions>;
    /// Gets the buckets the user has joined.
    fn get_buckets_user_is_a_part_of(&self, user_uuid: Uuid) -> QueryResult<Vec<Bucket>>;
    /// Gets the users in a given bucket.
    fn get_users_in_bucket(&self, bucket_uuid: Uuid) -> QueryResult<Vec<User>>;
}
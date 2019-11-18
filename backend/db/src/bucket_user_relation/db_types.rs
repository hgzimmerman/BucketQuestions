//! Database types for relations between buckets and users.
use crate::schema::bucket_user_relation;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A relation between users and buckets.
/// It also contains permissions for what users can do to the bucket.
#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize,
)]
#[primary_key(user_uuid, bucket_uuid)]
#[table_name = "bucket_user_relation"]
pub struct BucketUserRelation {
    /// The uuid of the user.
    pub user_uuid: Uuid,
    /// The uuid of the bucket.
    pub bucket_uuid: Uuid,
    /// Can the user set the visibility of the bucket.
    pub set_public_permission: bool,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: bool,
    /// Can the user set the bucket to private.
    pub set_exclusive_permission: bool,
    /// Can the user kick other users in the bucket.
    pub kick_permission: bool,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: bool,
    /// When the row was last updated.
    pub updated_at: NaiveDateTime,
    /// When the row was created.
    pub created_at: NaiveDateTime,
}

/// Structure used to create new join relations between users and buckets.
#[derive(Clone, Copy, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "bucket_user_relation"]
pub struct NewBucketUserRelation {
    /// The uuid of the user.
    pub user_uuid: Uuid,
    /// The uuid of the bucket.
    pub bucket_uuid: Uuid,
    /// Can the user set the visibility of the bucket.
    pub set_public_permission: bool,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: bool,
    /// Can the user set the bucket to private.
    pub set_exclusive_permission: bool,
    /// Can the user kick other users.
    pub kick_permission: bool,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: bool,
}
/// Structure used to create new join relations between users and buckets.
#[derive(Clone, Copy, AsChangeset, Identifiable, Debug, Serialize, Deserialize)]
#[primary_key(user_uuid, bucket_uuid)]
#[table_name = "bucket_user_relation"]
pub struct BucketUserPermissionsChangeset {
    /// The user id
    pub user_uuid: Uuid,
    /// the bucket uuid
    pub bucket_uuid: Uuid,
    /// Can the user set the visibility of the bucket.
    pub set_public_permission: Option<bool>,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: Option<bool>,
    /// Can the user set the bucket to private.
    pub set_exclusive_permission: Option<bool>,
    /// Can the user kick other users.
    pub kick_permission: Option<bool>,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: Option<bool>,
}


/// Structure that just contains the permissions for a user-bucket relation.
#[derive(Clone, Copy, Queryable, Debug, Serialize, Deserialize)]
pub struct BucketUserPermissions {
    /// Can the user set the visibility of the bucket.
    pub set_public_permission: bool,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: bool,
    /// Can the user make the bucket private
    pub set_exclusive_permission: bool,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: bool,
    /// Can the user kick other users.
    pub kick_permission: bool,
}


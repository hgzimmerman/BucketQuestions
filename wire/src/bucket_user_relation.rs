use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::user::User;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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


#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UserAndPermissions {
    /// The user
    pub user: User,
    /// The associated permissions
    pub permissions: BucketUserPermissions
}
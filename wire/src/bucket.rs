use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;


#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Bucket {
    /// The bucket's unique identifier within the application.
    pub uuid: Uuid,
    /// The name of the bucket that is shown to users.
    pub bucket_name: String,
    /// The slug that appears in the url.
    /// Must be unique.
    pub bucket_slug: String,
    /// Can users find it through the UI.
    pub public_viewable: bool,
    /// Is the bucket session currently active.
    pub drawing_enabled: bool,
    /// Can an unjoined user join the bucket.
    pub exclusive: bool,
    /// When the row was last updated.
    pub updated_at: NaiveDateTime,
    /// When the row was created.
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NewBucket {
    /// The name of the bucket
    pub bucket_name: String,
    /// The slug in the url for the bucket
    pub bucket_slug: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BucketFlagChangeset {
    /// Identifier of bucket
    pub uuid: Uuid,
    /// Is the bucket visible
    pub public_viewable: Option<bool>,
    /// Is the bucket session currently active.
    pub drawing_enabled: Option<bool>,
    /// Can an unjoined user join the bucket.
    pub exclusive: Option<bool>,
}

// Special types


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserUuidQueryParam {
    pub user_uuid: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetPermissionsRequest {
    pub target_user_uuid: Uuid,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeDrawingRequest {
    drawing: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeVisibilityRequest {
    visible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeBucketFlagsRequest {
    /// Is the bucket visible
    pub publicly_visible: Option<bool>,
    /// Is the bucket session currently active.
    pub drawing_enabled: Option<bool>,
    /// Can an unjoined user join the bucket.
    pub exclusive: Option<bool>,
}

/// Request to create a bucket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBucketRequest {
    pub bucket_name: String,
}

//! Module for bucket related database interactions.

use crate::schema::{
    answer, bucket, bucket_user_relation, question, user_question_favorite_relation,
};
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A struct representing a bucket.
/// A bucket is a session associated with questions.
#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "bucket"]
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

/// Structure used to create new users.
#[derive(Clone, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "bucket"]
pub struct NewBucket {
    /// The name of the bucket
    pub bucket_name: String,
    /// The slug in the url for the bucket
    pub bucket_slug: String,
}

/// A changeset for the bucket flags
#[derive(Clone, Copy, AsChangeset, Identifiable, Debug, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "bucket"]
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

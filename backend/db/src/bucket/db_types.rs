//! Module for bucket related database interactions.

use crate::schema::{answer, bucket_user_relation, bucket, question, user_question_favorite_relation};
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

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
    pub visible: bool,
    /// Is the bucket session currently active.
    pub drawing_enabled: bool,
    /// Can an unjoined user join the bucket.
    pub private: bool,
    /// When the row was last updated.
    pub updated_at: NaiveDateTime,
    /// When the row was created.
    pub created_at: NaiveDateTime
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
    pub visible: Option<bool>,
    /// Is the bucket session currently active.
    pub drawing_enabled: Option<bool>,
    /// Can an unjoined user join the bucket.
    pub private: Option<bool>,
}

/// A relation between users and buckets.
/// It also contains permissions for what users can do to the bucket.
#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize,
)]
#[primary_key(user_uuid, bucket_uuid)]
#[table_name = "bucket_user_relation"]
pub struct BucketUserJoin {
    /// The uuid of the user.
    pub user_uuid: Uuid,
    /// The uuid of the bucket.
    pub bucket_uuid: Uuid,
    /// Can the user set the visibility of the bucket.
    pub set_visibility_permission: bool,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: bool,
    /// Can the user set the bucket to private.
    pub set_private_permission: bool,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: bool,
    /// When the row was last updated.
    pub updated_at: NaiveDateTime,
    /// When the row was created.
    pub created_at: NaiveDateTime
}

// TODO, remove this because it isn't distinct from the Bucket User Join
/// Structure used to create new join relations between users and buckets.
#[derive(Clone, Copy, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "bucket_user_relation"]
pub struct NewBucketUserJoin {
    /// The uuid of the user.
    pub user_uuid: Uuid,
    /// The uuid of the bucket.
    pub bucket_uuid: Uuid,
    /// Can the user set the visibility of the bucket.
    pub set_visibility_permission: bool,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: bool,
    /// Can the user set the bucket to private.
    pub set_private_permission: bool,
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
    pub set_visibility_permission: Option<bool>,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: Option<bool>,
    /// Can the user set the bucket to private.
    pub set_private_permission: Option<bool>,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: Option<bool>,
}

/// Structure that just contains the permissions for a user-bucket relation.
#[derive(Clone, Copy, Queryable, Debug, Serialize, Deserialize)]
pub struct BucketUserPermissions {
    /// Can the user set the visibility of the bucket.
    pub set_visibility_permission: bool,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: bool,
    /// Can the user make the bucket private
    pub set_private_permission: bool,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: bool,
}

/// A struct representing a question.
#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "question"]
pub struct Question {
    /// The question's unique identifier within the application.
    pub uuid: Uuid,
    /// The bucket to which the question belongs.
    pub bucket_uuid: Uuid,
    /// The user that made the question.
    pub user_uuid: Option<Uuid>,
    /// The content of the question.
    pub question_text: String,
    /// Is the question no longer in the metaphorical bucket.
    /// The question is still associated with the bucket,
    /// but it can't be randomly drawn unless it is explicitly
    /// put back in the bucket.
    /// The archived flag is a formalization of the question being on the floor.
    pub archived: bool,
    /// When the row was last updated.
    pub updated_at: NaiveDateTime,
    /// When the row was created.
    pub created_at: NaiveDateTime
}

/// A struct for creating new questions.
#[derive(Clone, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "question"]
pub struct NewQuestion {
    /// The bucket to which the question belongs.
    pub bucket_uuid: Uuid,
    /// The user that made the question.
    pub user_uuid: Option<Uuid>,
    /// The content of the question.
    pub question_text: String,
}

/// A struct for recording answers.
#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "answer"]
pub struct Answer {
    /// The unique identifier for the answer
    pub uuid: Uuid,
    /// The user who answered
    pub user_uuid: Option<Uuid>,
    /// The question to which the answer is responding.
    pub question_uuid: Uuid,
    /// Can the outside world see the answer.
    pub publicly_visible: bool,
    /// The answer
    pub answer_text: String,
    /// When the row was last updated.
    pub updated_at: NaiveDateTime,
    /// When the row was created.
    pub created_at: NaiveDateTime
}

/// A struct for creating new answers
#[derive(Clone, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "answer"]
pub struct NewAnswer {
    /// The user who made the answer
    pub user_uuid: Option<Uuid>,
    /// The question to which the answer is responding.
    pub question_uuid: Uuid,
    /// Can the outside world see the answer.
    pub publicly_visible: bool,
    /// The answer
    pub answer_text: String,
}

/// A relation for recording user's favorite questions.
#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize,
)]
#[primary_key(user_uuid, question_uuid)]
#[table_name = "user_question_favorite_relation"]
pub struct FavoriteQuestionRelation {
    /// User
    pub user_uuid: Uuid,
    /// Question
    pub question_uuid: Uuid,
    /// When the row was last updated.
    pub updated_at: NaiveDateTime,
    /// When the row was created.
    pub created_at: NaiveDateTime
}

/// Structure for creating a new favorite relation.
#[derive(Clone, Copy, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "user_question_favorite_relation"]
pub struct NewFavoriteQuestionRelation {
    /// User
    pub user_uuid: Uuid,
    /// Question
    pub question_uuid: Uuid,

}

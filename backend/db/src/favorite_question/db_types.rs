//! Module for favorite related database interactions.
use crate::schema::user_question_favorite_relation;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub created_at: NaiveDateTime,
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

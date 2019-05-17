//! Module for answer related database interactions.
use crate::schema::answer;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    /// This is in contrast to if just the user themselves can see the answer.
    pub publicly_visible: bool,
    /// The answer
    pub answer_text: String,
    /// When the row was last updated.
    pub updated_at: NaiveDateTime,
    /// When the row was created.
    pub created_at: NaiveDateTime,
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

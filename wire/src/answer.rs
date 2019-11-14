use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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

//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnswerRequest {
    /// The question to which the answer is responding.
    pub question_uuid: Uuid,
    /// Can the outside world see the answer.
    pub publicly_visible: bool,
    /// The answer
    pub answer_text: String,
}

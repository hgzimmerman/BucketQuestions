use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;


#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NewQuestion {
    /// The bucket to which the question belongs.
    pub bucket_uuid: Uuid,
    /// The user that made the question.
    pub user_uuid: Option<Uuid>,
    /// The content of the question.
    pub question_text: String,
}

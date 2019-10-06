//! Answer conversions

use wire;
use crate::answer::db_types::{Answer, NewAnswer};

impl Into<wire::answer::Answer> for Answer {
    fn into(self) -> wire::answer::Answer {
        wire::answer::Answer {
            uuid: self.uuid,
            user_uuid: self.user_uuid,
            question_uuid: self.question_uuid,
            publicly_visible: self.publicly_visible,
            answer_text: self.answer_text,
            updated_at: self.updated_at,
            created_at: self.created_at
        }
    }
}

impl From<wire::answer::Answer> for Answer {
    fn from(answer: wire::answer::Answer) -> Self {
        Answer {
            uuid: answer.uuid,
            user_uuid: answer.user_uuid,
            question_uuid: answer.question_uuid,
            publicly_visible: answer.publicly_visible,
            answer_text: answer.answer_text,
            updated_at: answer.updated_at,
            created_at: answer.created_at
        }
    }
}

impl Into<wire::answer::NewAnswer> for NewAnswer {
    fn into(self) -> wire::answer::NewAnswer {
        wire::answer::NewAnswer {
            user_uuid: self.user_uuid,
            question_uuid: self.question_uuid,
            publicly_visible: self.publicly_visible,
            answer_text: self.answer_text
        }
    }
}

impl From<wire::answer::NewAnswer> for NewAnswer {
    fn from(new_answer: wire::answer::NewAnswer) -> Self {
        NewAnswer {
            user_uuid: new_answer.user_uuid,
            question_uuid: new_answer.question_uuid,
            publicly_visible: new_answer.publicly_visible,
            answer_text: new_answer.answer_text
        }

    }
}
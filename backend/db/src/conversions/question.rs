//! Question conversions.

use wire;
use crate::question::db_types::{NewQuestion, Question};

impl Into<wire::question::Question> for Question {
    fn into(self) -> wire::question::Question {
        wire::question::Question {
            uuid: self.uuid,
            bucket_uuid: self.bucket_uuid,
            user_uuid: self.user_uuid,
            question_text: self.question_text,
            archived: self.archived,
            updated_at: self.updated_at,
            created_at: self.created_at
        }
    }
}

impl From<wire::question::Question> for Question {
    fn from(question: wire::question::Question) -> Self {
        Question {
            uuid: question.uuid,
            bucket_uuid: question.bucket_uuid,
            user_uuid: question.user_uuid,
            question_text: question.question_text,
            archived: question.archived,
            updated_at: question.updated_at,
            created_at: question.created_at
        }
    }
}

impl Into<wire::question::NewQuestion> for NewQuestion {
    fn into(self) -> wire::question::NewQuestion {
        wire::question::NewQuestion {
            bucket_uuid: self.bucket_uuid,
            user_uuid: self.user_uuid,
            question_text: self.question_text
        }
    }
}

impl From<wire::question::NewQuestion> for NewQuestion {
    fn from(new_question: wire::question::NewQuestion) -> Self {
        NewQuestion {
            bucket_uuid: new_question.bucket_uuid,
            user_uuid: new_question.user_uuid,
            question_text: new_question.question_text
        }
    }
}
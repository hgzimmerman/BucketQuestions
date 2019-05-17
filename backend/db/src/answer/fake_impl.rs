//! Mock impl

use crate::{
    answer::{
        db_types::{Answer, NewAnswer},
        interface::AnswerRepository,
    },
    fake::{DummyDbErrorInfo, FakeDatabase},
};
use diesel::result::{DatabaseErrorKind, Error};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

impl AnswerRepository for Arc<Mutex<FakeDatabase>> {
    fn create_answer(&self, answer: NewAnswer) -> Result<Answer, Error> {
        let uuid = Uuid::new_v4();
        let answer = Answer {
            uuid,
            user_uuid: answer.user_uuid,
            question_uuid: answer.question_uuid,
            publicly_visible: answer.publicly_visible,
            answer_text: answer.answer_text,
            updated_at: chrono::Utc::now().naive_utc(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        let mut db = self.lock().unwrap();
        if db.answers.iter().find(|q| q.uuid == uuid).is_some() {
            return Err(Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                Box::new(DummyDbErrorInfo::new()),
            ));
        }
        db.answers.push(answer.clone());
        return Ok(answer);
    }

    fn delete_answer(&self, uuid: Uuid) -> Result<Answer, Error> {
        let mut db = self.lock().unwrap();
        let index = db
            .answers
            .iter()
            .position(|a| a.uuid == uuid)
            .ok_or_else(|| Error::NotFound)?;
        Ok(db.answers.remove(index))
    }

    fn get_answers_for_question(
        &self,
        question_uuid: Uuid,
        visibility_required: bool,
    ) -> Result<Vec<Answer>, Error> {
        let db = self.lock().unwrap();
        if visibility_required {
            let answers = db
                .answers
                .iter()
                .filter(|a| a.question_uuid == question_uuid && a.publicly_visible)
                .cloned()
                .collect();
            Ok(answers)
        } else {
            // just get all answers
            let answers = db
                .answers
                .iter()
                .filter(|a| a.question_uuid == question_uuid)
                .cloned()
                .collect();
            Ok(answers)
        }
    }
}

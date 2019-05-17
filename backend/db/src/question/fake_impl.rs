//! Mock impl

use crate::{
    fake::{DummyDbErrorInfo, FakeDatabase},
    question::{
        db_types::{NewQuestion, Question},
        interface::QuestionRepository,
    },
};
use diesel::result::{DatabaseErrorKind, Error};
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

impl QuestionRepository for Arc<Mutex<FakeDatabase>> {
    fn create_question(&self, question: NewQuestion) -> Result<Question, Error> {
        let uuid = Uuid::new_v4();
        let question = Question {
            uuid,
            bucket_uuid: question.bucket_uuid,
            user_uuid: question.user_uuid,
            question_text: question.question_text,
            archived: false,
            updated_at: chrono::Utc::now().naive_utc(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        let mut db = self.lock().unwrap();
        if db.questions.iter().find(|q| q.uuid == uuid).is_some() {
            return Err(Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                Box::new(DummyDbErrorInfo::new()),
            ));
        }
        db.questions.push(question.clone());
        return Ok(question);
    }

    fn delete_question(&self, uuid: Uuid) -> Result<Question, Error> {
        let mut db = self.lock().unwrap();
        let index = db
            .questions
            .iter()
            .position(|q| q.uuid == uuid)
            .ok_or_else(|| Error::NotFound)?;
        Ok(db.questions.remove(index))
    }

    fn get_random_question(&self, bucket_uuid: Uuid) -> Result<Option<Question>, Error> {
        let db = self.lock().unwrap();
        let bucket_questions: Vec<&Question> = db
            .questions
            .iter()
            .filter(|q| q.bucket_uuid == bucket_uuid)
            .collect();
        if bucket_questions.len() > 0 {
            let index: usize = thread_rng().gen_range(0, bucket_questions.len());
            Ok(bucket_questions.get(index).cloned().cloned())
        } else {
            Ok(None)
        }
    }

    fn get_number_of_active_questions_for_bucket(&self, bucket_uuid: Uuid) -> Result<i64, Error> {
        let db = self.lock().unwrap();
        let count = db
            .questions
            .iter()
            .filter(|q| !q.archived && q.bucket_uuid == bucket_uuid)
            .count();
        Ok(count as i64)
    }

    fn get_all_questions_for_bucket_of_given_archived_status(
        &self,
        bucket_uuid: Uuid,
        archived: bool,
    ) -> Result<Vec<Question>, Error> {
        let db = self.lock().unwrap();
        let questions = db
            .questions
            .iter()
            .filter(|q| q.archived == archived && q.bucket_uuid == bucket_uuid)
            .cloned()
            .collect();
        Ok(questions)
    }

    fn set_archive_status_for_question(
        &self,
        question_uuid: Uuid,
        archived: bool,
    ) -> Result<Question, Error> {
        let mut db = self.lock().unwrap();
        let question = db
            .questions
            .iter_mut()
            .find(|q| q.uuid == question_uuid)
            .ok_or_else(|| Error::NotFound)?;
        question.archived = archived;
        Ok(question.clone())
    }
}

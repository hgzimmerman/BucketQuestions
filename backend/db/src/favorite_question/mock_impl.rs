//! Mock impl

use crate::{
    favorite_question::{
        db_types::{FavoriteQuestionRelation, NewFavoriteQuestionRelation},
        interface::FavoriteQuestionRelationRepository,
    },
    mock::{DummyDbErrorInfo, MockDatabase},
    question::db_types::Question,
};
use diesel::result::{DatabaseErrorKind, Error};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

impl FavoriteQuestionRelationRepository for Arc<Mutex<MockDatabase>> {
    fn favorite_question(&self, relation: NewFavoriteQuestionRelation) -> Result<(), Error> {
        let mut db = self.lock().unwrap();
        let relation = FavoriteQuestionRelation {
            user_uuid: relation.user_uuid,
            question_uuid: relation.question_uuid,
            updated_at: chrono::Utc::now().naive_utc(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        if db
            .favorite_question_relations
            .iter()
            .find(|r| {
                r.user_uuid == relation.user_uuid && r.question_uuid == relation.question_uuid
            })
            .is_some()
        {
            return Err(Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                Box::new(DummyDbErrorInfo::new()),
            ));
        }
        db.favorite_question_relations.push(relation.clone());
        return Ok(());
    }

    fn unfavorite_question(&self, relation: NewFavoriteQuestionRelation) -> Result<(), Error> {
        let mut db = self.lock().unwrap();
        let index = db
            .favorite_question_relations
            .iter()
            .position(|r| {
                r.user_uuid == relation.user_uuid && r.question_uuid == relation.question_uuid
            })
            .ok_or_else(|| Error::NotFound)?;
        db.questions.remove(index);
        Ok(())
    }

    fn get_favorite_questions(&self, user_uuid: Uuid) -> Result<Vec<Question>, Error> {
        let db = self.lock().unwrap();
        let question_uuids: Vec<Uuid> = db
            .favorite_question_relations
            .iter()
            .filter(|f| f.user_uuid == user_uuid)
            .map(|f| f.question_uuid)
            .collect();
        let questions = db
            .questions
            .iter()
            .filter(|q| question_uuids.iter().any(|uuid| &q.uuid == uuid))
            .cloned()
            .collect();
        Ok(questions)
    }
}

//! Specification of functions.
use crate::answer::db_types::{Answer, NewAnswer};
use diesel::QueryResult;
use uuid::Uuid;

/// Functions for specifically working with Answers.
pub trait AnswerRepository {
    /// Create an answer
    fn create_answer(&self, answer: NewAnswer) -> QueryResult<Answer>;
    /// Delete an answer
    fn delete_answer(&self, uuid: Uuid) -> QueryResult<Answer>;
    /// Gets answers for the question.
    /// This should only be the publicly visible answers.
    fn get_answers_for_question(
        &self,
        question_uuid: Uuid,
        visibility_required: bool,
    ) -> QueryResult<Vec<Answer>>;
}

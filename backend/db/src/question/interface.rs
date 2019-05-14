//! Specification of functions.
use crate::question::db_types::{NewQuestion, Question};
use diesel::QueryResult;
use uuid::Uuid;

/// Functions for specifically working with questions.
pub trait QuestionRepository {
    /// Create a question
    fn create_question(&self, question: NewQuestion) -> QueryResult<Question>;
    /// Delete question
    fn delete_question(&self, uuid: Uuid) -> QueryResult<Question>;
    /// Gets a random question.
    fn get_random_question(&self, bucket_uuid: Uuid) -> QueryResult<Option<Question>>;
    /// Gets the number of active questions.
    fn get_number_of_active_questions_for_bucket(&self, bucket_uuid: Uuid) -> QueryResult<i64>;
    /// Gets all questions for a bucket of a specified archived state.
    fn get_all_questions_for_bucket_of_given_archived_status(
        &self,
        bucket_uuid: Uuid,
        archived: bool,
    ) -> QueryResult<Vec<Question>>;
    /// Disable or Enable the question from drawing eligibility.
    fn set_archive_status_for_question(
        &self,
        question_uuid: Uuid,
        archived: bool,
    ) -> QueryResult<Question>;
}

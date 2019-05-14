//! Implementation of the specified interfaces for PgConnection.
use crate::{
    question::{
        db_types::{NewQuestion, Question},
        interface::QuestionRepository,
    },
    schema::question,
    AsConnRef,
};
use diesel::{
    query_dsl::RunQueryDsl, result::Error, BoolExpressionMethods, ExpressionMethods,
    OptionalExtension, QueryDsl,
};
use uuid::Uuid;

impl<T> QuestionRepository for T
where
    T: AsConnRef,
{
    fn create_question(&self, question: NewQuestion) -> Result<Question, Error> {
        crate::util::create_row(question::table, question, self.as_conn())
    }

    fn delete_question(&self, uuid: Uuid) -> Result<Question, Error> {
        crate::util::delete_row(question::table, uuid, self.as_conn())
    }

    fn get_random_question(&self, bucket_uuid: Uuid) -> Result<Option<Question>, Error> {
        no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

        // Get a question in the bucket, that isn't on the floor.
        let condition = question::bucket_uuid
            .eq(bucket_uuid)
            .and(question::archived.eq(false));

        question::table
            .filter(condition)
            .order(RANDOM)
            .first(self.as_conn())
            .optional()
    }

    fn get_number_of_active_questions_for_bucket(&self, bucket_uuid: Uuid) -> Result<i64, Error> {
        question::table
            .filter(
                question::bucket_uuid
                    .eq(bucket_uuid)
                    .and(question::archived.eq(false)),
            )
            .count()
            .get_result(self.as_conn())
    }

    fn get_all_questions_for_bucket_of_given_archived_status(
        &self,
        bucket_uuid: Uuid,
        archived: bool,
    ) -> Result<Vec<Question>, Error> {
        question::table
            .filter(
                question::bucket_uuid
                    .eq(bucket_uuid)
                    .and(question::archived.eq(archived)),
            )
            .order_by(question::updated_at)
            .get_results(self.as_conn())
    }

    fn set_archive_status_for_question(
        &self,
        question_uuid: Uuid,
        archived: bool,
    ) -> Result<Question, Error> {
        let target = question::table.find(question_uuid);

        diesel::update(target)
            .set(question::archived.eq(archived))
            .get_result(self.as_conn())
    }
}

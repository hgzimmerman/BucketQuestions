//! Implementation of the specified interfaces for PgConnection.

use crate::{
    answer::{
        db_types::{Answer, NewAnswer},
        interface::AnswerRepository,
    },
    schema::answer,
    AsConnRef,
};
use diesel::{
    query_dsl::QueryDsl, result::Error, BoolExpressionMethods, ExpressionMethods, RunQueryDsl,
};
use uuid::Uuid;

impl<T> AnswerRepository for T
where
    T: AsConnRef,
{
    fn create_answer(&self, answer: NewAnswer) -> Result<Answer, Error> {
        crate::util::create_row(answer::table, answer, self.as_conn())
    }

    fn delete_answer(&self, uuid: Uuid) -> Result<Answer, Error> {
        crate::util::delete_row(answer::table, uuid, self.as_conn())
    }

    fn get_answers_for_question(
        &self,
        question_uuid: Uuid,
        visibility_required: bool,
    ) -> Result<Vec<Answer>, Error> {
        if visibility_required {
            answer::table
                .filter(
                    answer::question_uuid
                        .eq(question_uuid)
                        .and(answer::publicly_visible.eq(true)),
                )
                .order_by(answer::updated_at)
                .get_results(self.as_conn())
        } else {
            // gets both private and public
            answer::table
                .filter(answer::question_uuid.eq(question_uuid))
                .order_by(answer::updated_at)
                .get_results(self.as_conn())
        }
    }
}

//! Implementation of the specified interfaces for PgConnection.
use crate::{
    favorite_question::{
        db_types::{FavoriteQuestionRelation, NewFavoriteQuestionRelation},
        interface::FavoriteQuestionRelationRepository,
    },
    question::db_types::Question,
    schema::{question, user_question_favorite_relation},
    AsConnRef,
};
use diesel::{
    query_dsl::{QueryDsl, RunQueryDsl},
    result::Error,
    BoolExpressionMethods, ExpressionMethods,
};
use uuid::Uuid;

impl<T> FavoriteQuestionRelationRepository for T
where
    T: AsConnRef,
{
    fn favorite_question(&self, relation: NewFavoriteQuestionRelation) -> Result<(), Error> {
        crate::util::create_row(
            user_question_favorite_relation::table,
            relation,
            self.as_conn(),
        )
        .map(|_: FavoriteQuestionRelation| ())
    }

    fn unfavorite_question(&self, relation: NewFavoriteQuestionRelation) -> Result<(), Error> {
        let target = user_question_favorite_relation::table.filter(
            user_question_favorite_relation::user_uuid
                .eq(relation.user_uuid)
                .and(user_question_favorite_relation::question_uuid.eq(relation.question_uuid)),
        );
        diesel::delete(target).execute(self.as_conn()).map(|_| ())
    }

    fn get_favorite_questions(&self, user_uuid: Uuid) -> Result<Vec<Question>, Error> {
        use user_question_favorite_relation as favorite;
        favorite::table
            .filter(favorite::user_uuid.eq(user_uuid))
            .select(favorite::question_uuid)
            .inner_join(question::table)
            .order_by(favorite::updated_at)
            .select(question::all_columns)
            .get_results(self.as_conn())
    }
}

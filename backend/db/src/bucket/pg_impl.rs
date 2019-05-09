//! Implementation of the specified interfaces for PgConnection.

use crate::{
    bucket::{
        db_types::{
            Answer, Bucket, BucketFlagChangeset, BucketUserRelation, BucketUserPermissions,
            BucketUserPermissionsChangeset, FavoriteQuestionRelation, NewAnswer, NewBucket,
            NewBucketUserRelation, NewFavoriteQuestionRelation, NewQuestion, Question,
        },
        interface::{
            AnswerRepository, BucketRepository, BucketUserRelationRepository,
            FavoriteQuestionRelationRepository, QuestionRepository,
        },
    },
    diesel::OptionalExtension,
    schema::{answer, bucket_user_relation, bucket, question, user_question_favorite_relation, bq_user},
    user::User,
};
use diesel::{
    pg::PgConnection,
    query_dsl::{QueryDsl, RunQueryDsl},
    result::Error,
    BoolExpressionMethods, ExpressionMethods, SaveChangesDsl,
};
use log::info;
use uuid::Uuid;

impl BucketRepository for PgConnection {
    fn create_bucket(&self, new_bucket: NewBucket) -> Result<Bucket, Error> {
        crate::util::create_row(bucket::table, new_bucket, self)
    }

    fn delete_bucket(&self, bucket_uuid: Uuid) -> Result<Bucket, Error> {
        crate::util::delete_row(bucket::table, bucket_uuid, self)
    }

    fn get_publicly_visible_buckets(&self) -> Result<Vec<Bucket>, Error> {
        bucket::table
            .filter(bucket::public_viewable.eq(true))
            .get_results(self)
    }

    fn get_bucket_by_slug(&self, slug: String) -> Result<Bucket, Error> {
        bucket::table
            .filter(&bucket::bucket_slug.eq(slug))
            .first(self)
    }

    fn get_bucket_by_uuid(&self, uuid: Uuid) -> Result<Bucket, Error> {
        crate::util::get_row(bucket::table, uuid, self)
    }

    fn change_bucket_flags(&self, changeset: BucketFlagChangeset) -> Result<Bucket, Error> {
        changeset.save_changes(self)
            .or_else(|error: Error| {
                // The query will return an error if there are no changes,
                // if that is the case, just fetch the whole bucket.
                match error {
                    Error::QueryBuilderError(_) => {
                        self.get_bucket_by_uuid(changeset.uuid)
                    }
                    other => Err(other)
                }
            })
    }
}

impl BucketUserRelationRepository for PgConnection {
    fn add_user_to_bucket(&self, relation: NewBucketUserRelation) -> Result<BucketUserRelation, Error> {
        crate::util::create_row(bucket_user_relation::table, relation, self)
    }

    fn remove_user_from_bucket(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> Result<BucketUserRelation, Error> {
        let target = bucket_user_relation::table.filter(
            bucket_user_relation::user_uuid
                .eq(user_uuid)
                .and(bucket_user_relation::bucket_uuid.eq(bucket_uuid)),
        );
        diesel::delete(target).get_result(self)
    }

    fn set_permissions(
        &self,
        permissions_changeset: BucketUserPermissionsChangeset,
    ) -> Result<BucketUserRelation, Error> {
        permissions_changeset.save_changes(self)
    }

    fn get_permissions(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> Result<BucketUserPermissions, Error> {
        bucket_user_relation::table
            .filter(
                bucket_user_relation::user_uuid
                    .eq(user_uuid)
                    .and(bucket_user_relation::bucket_uuid.eq(bucket_uuid)),
            )
            .select((
                bucket_user_relation::set_public_permission,
                bucket_user_relation::set_drawing_permission,
                bucket_user_relation::set_exclusive_permission,
                bucket_user_relation::grant_permissions_permission,
            ))
            .get_result::<BucketUserPermissions>(self)
    }

    fn get_buckets_user_is_a_part_of(&self, user_uuid: Uuid) -> Result<Vec<Bucket>, Error> {
        info!("get_buckets_user_is_a_part_of");
        bucket_user_relation::table
            .filter(bucket_user_relation::user_uuid.eq(user_uuid))
            .select(bucket_user_relation::bucket_uuid)
            .inner_join(bucket::table)
            .select(bucket::all_columns)
            .get_results(self)
    }

    fn get_users_in_bucket(&self, bucket_uuid: Uuid) -> Result<Vec<User>, Error> {
        info!("get_users_in_bucket");
        bucket_user_relation::table
            .filter(bucket_user_relation::bucket_uuid.eq(bucket_uuid))
            .select(bucket_user_relation::user_uuid)
            .inner_join(bq_user::table)
            .select(bq_user::all_columns)
            .get_results(self)
    }
}

impl QuestionRepository for PgConnection {
    fn create_question(&self, question: NewQuestion) -> Result<Question, Error> {
        crate::util::create_row(question::table, question, self)
    }

    fn delete_question(&self, uuid: Uuid) -> Result<Question, Error> {
        crate::util::delete_row(question::table, uuid, self)
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
            .first(self)
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
            .get_result(self)
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
            .get_results(self)
    }

    fn set_archive_status_for_question(
        &self,
        question_uuid: Uuid,
        archived: bool,
    ) -> Result<Question, Error> {
        let target = question::table.find(question_uuid);

        diesel::update(target)
            .set(question::archived.eq(archived))
            .get_result(self)
    }
}

impl AnswerRepository for PgConnection {
    fn create_answer(&self, answer: NewAnswer) -> Result<Answer, Error> {
        crate::util::create_row(answer::table, answer, self)
    }

    fn delete_answer(&self, uuid: Uuid) -> Result<Answer, Error> {
        crate::util::delete_row(answer::table, uuid, self)
    }

    fn get_answers_for_question(&self, question_uuid: Uuid) -> Result<Vec<Answer>, Error> {
        answer::table
            .filter(answer::question_uuid.eq(question_uuid))
            .get_results(self)
    }
}

impl FavoriteQuestionRelationRepository for PgConnection {
    fn favorite_question(&self, relation: NewFavoriteQuestionRelation) -> Result<(), Error> {
        crate::util::create_row(user_question_favorite_relation::table, relation, self)
            .map(|_: FavoriteQuestionRelation| ())
    }

    fn unfavorite_question(&self, relation: NewFavoriteQuestionRelation) -> Result<(), Error> {
        let target = user_question_favorite_relation::table.filter(
            user_question_favorite_relation::user_uuid
                .eq(relation.user_uuid)
                .and(user_question_favorite_relation::question_uuid.eq(relation.question_uuid)),
        );
        diesel::delete(target).execute(self).map(|_| ())
    }

    fn get_favorite_questions(&self, user_uuid: Uuid) -> Result<Vec<Question>, Error> {
        use user_question_favorite_relation  as favorite;
        favorite::table
            .filter(favorite::user_uuid.eq(user_uuid))
            .select(favorite::question_uuid)
            .inner_join(question::table)
            .select(question::all_columns)
            .get_results(self)
    }
}

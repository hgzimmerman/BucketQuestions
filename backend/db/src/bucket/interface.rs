//! Specification of what functions are supported for storing data for buckets.
//!
//! These traits should try to not include significant quantities of business logic.
//! It should try to deal with only the types specified in db_types, and avoid wire types.

use crate::{
    bucket::db_types::{
        Answer, Bucket, BucketFlagChangeset, BucketUserRelation, BucketUserPermissions,
        BucketUserPermissionsChangeset, NewAnswer, NewBucket, NewBucketUserRelation,
        NewFavoriteQuestionRelation, NewQuestion, Question,
    },
    user::User,
};
use diesel::QueryResult;
use uuid::Uuid;

/// Functions for specifically working with buckets
pub trait BucketRepository {
    /// Create a bucket.
    fn create_bucket(&self, new_bucket: NewBucket) -> QueryResult<Bucket>;
    /// Delete a bucket.
    fn delete_bucket(&self, bucket_uuid: Uuid) -> QueryResult<Bucket>;
    /// Gets all publicly visible buckets.
    fn get_publicly_visible_buckets(&self) -> QueryResult<Vec<Bucket>>;
    /// Gets the bucket via its slug.
    fn get_bucket_by_slug(&self, slug: String) -> QueryResult<Bucket>;
    /// Gets the bucket via its uuid.
    fn get_bucket_by_uuid(&self, uuid: Uuid) -> QueryResult<Bucket>;
    /// Change the blags that govern the buckets behavior
    fn change_bucket_flags(&self, changeset: BucketFlagChangeset) -> QueryResult<Bucket>;
}

/// Functions for specifically working with bucket user relations.
pub trait BucketUserRelationRepository {
    /// Adds a user to the bucket.
    fn add_user_to_bucket(&self, relation: NewBucketUserRelation) -> QueryResult<BucketUserRelation>;
    /// Removes the user from bucket.
    fn remove_user_from_bucket(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> QueryResult<BucketUserRelation>;
    /// Get the bucket user relation
    fn get_user_bucket_relation(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid
    ) -> QueryResult<BucketUserRelation>;
    /// Set permissions for the user-bucket relation.
    fn set_permissions(
        &self,
        permissions_changeset: BucketUserPermissionsChangeset,
    ) -> QueryResult<BucketUserRelation>;
    /// Get the permissions for the user.
    /// The user may not be a part of the bucket.
    fn get_permissions(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> QueryResult<BucketUserPermissions>;
    /// Gets the buckets the user has joined.
    fn get_buckets_user_is_a_part_of(&self, user_uuid: Uuid) -> QueryResult<Vec<Bucket>>;
    /// Gets the users in a given bucket.
    fn get_users_in_bucket(&self, bucket_uuid: Uuid) -> QueryResult<Vec<User>>;
}

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

/// Functions for specifically working with Answers.
pub trait AnswerRepository {
    /// Create an answer
    fn create_answer(&self, answer: NewAnswer) -> QueryResult<Answer>;
    /// Delete an answer
    fn delete_answer(&self, uuid: Uuid) -> QueryResult<Answer>;
    /// Gets answers for the question.
    /// This should only be the publicly visible answers.
    fn get_answers_for_question(&self, question_uuid: Uuid, visibility_required: bool) -> QueryResult<Vec<Answer>>;
}

/// Functions for specifically working with Favorites.
pub trait FavoriteQuestionRelationRepository {
    /// Add the relation
    fn favorite_question(&self, relation: NewFavoriteQuestionRelation) -> QueryResult<()>;
    /// Removes the relation
    fn unfavorite_question(&self, relation: NewFavoriteQuestionRelation) -> QueryResult<()>;
    /// Gets the favorite quesitons.
    fn get_favorite_questions(&self, user_uuid: Uuid) -> QueryResult<Vec<Question>>;
}

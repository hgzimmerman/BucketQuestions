//! Specification of what functions are supported for storing data for buckets.
//!
//! These traits should try to not include significant quantities of business logic.
//! It should try to deal with only the types specified in db_types, and avoid wire types.

use crate::bucket::db_types::{Bucket, NewBucket, NewBucketUserJoin, BucketUserPermissionsChangeset, BucketUserJoin, BucketUserPermissions, NewQuestion, Question, NewAnswer, Answer, FavoriteQuestionRelation, NewFavoriteQuestionRelation};
use uuid::Uuid;
use diesel::QueryResult;

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
    /// Change the public visibility of the bucket.
    fn change_visibility(&self, bucket_uuid: Uuid, visible: bool) -> QueryResult<Bucket>;
    /// Allow the bucket to be drawn from.
    fn change_drawing_status(&self, bucket_uuid: Uuid, drawing: bool) -> QueryResult<Bucket>;
}

/// Functions for specifically working with bucket user relations.
pub trait BucketUserRelationRepository {
    /// Adds a user to the bucket.
    fn add_user_to_bucket(&self, relation: NewBucketUserJoin) -> QueryResult<BucketUserJoin>;
    /// Removes the user from bucket.
    fn remove_user_from_bucket(&self, user_uuid: Uuid, bucket_uuid: Uuid) -> QueryResult<BucketUserJoin>;
    /// Set permissions for the user-bucket relation.
    fn set_permissions(&self, permissions_changeset: BucketUserPermissionsChangeset) -> QueryResult<BucketUserJoin>;
    /// Get the permissions for the user.
    /// The user may not be a part of the bucket.
    fn get_permissions(&self, user_uuid: Uuid, bucket_uuid: Uuid) -> QueryResult<BucketUserPermissions>;
    /// Gets the buckets the user has joined.
    fn get_buckets_user_is_a_part_of(&self, user_uuid: Uuid) -> QueryResult<Vec<Bucket>>;
}


/// Functions for specifically working with questions.
pub trait QuestionRepository {
    /// Create a question
    fn create_question(&self, question: NewQuestion) -> QueryResult<Question>;
    /// Delete question
    fn delete_question(&self, uuid: Uuid) -> QueryResult<Question>;
    /// Gets a random question.
    fn get_random_question(&self, bucket_uuid: Uuid) -> QueryResult<Question>;
    /// Gets the number of active questions.
    fn get_number_of_active_questions(&self, bucket_uuid: Uuid) -> u32;
    /// Gets all active questions
    fn get_all_active_questions(&self, bucket_uuid: Uuid) -> QueryResult<Vec<Question>>;
    /// Remove the question from drawing eligibility.
    fn archive_question(&self, question_uuid: Uuid) -> QueryResult<Question>;
    /// Allow the question to be drawn again
    fn unarchive_question(&self, question_uuid: Uuid) -> QueryResult<Question>;
}


/// Functions for specifically working with Answers.
pub trait AnswerRepository {
    /// Create an answer
    fn create_answer(&self, answer: NewAnswer) -> QueryResult<Answer>;
    /// Delete an answer
    fn delete_answer(&self, uuid: Uuid) -> QueryResult<Answer>;
    /// Gets answers for the question.
    /// This should only be the publicly visible answers.
    fn get_answers_for_question(&self, question_uuid: Uuid) -> QueryResult<Vec<Answer>>;
}


/// Functions for specifically working with Favorites.
pub trait FavoriteQuestionRelationRepository {
    /// Add the relation
    fn favorite_question(&self, relation: NewFavoriteQuestionRelation);
    /// Removes the relation
    fn unfavorite_question(&self, relation: NewFavoriteQuestionRelation);
    /// Gets the favorite quesitons.
    fn get_favorite_questions(&self, user_uuid: Uuid) -> QueryResult<FavoriteQuestionRelation>;
}

//! Module for the database mock object.
use crate::{
    answer::db_types::Answer, bucket::db_types::Bucket,
    bucket_user_relation::db_types::BucketUserRelation,
    favorite_question::db_types::FavoriteQuestionRelation, question::db_types::Question,
    user::db_types::User,
};
use diesel::result::DatabaseErrorInformation;

/// This isn't expected to match on the info provided by the actual database.
///
#[derive(Clone, Copy, Debug)]
pub struct DummyDbErrorInfo;
impl DummyDbErrorInfo {
    /// Creates a new DummyDbErrorInfo
    pub fn new() -> Self {
        DummyDbErrorInfo
    }
}

impl DatabaseErrorInformation for DummyDbErrorInfo {
    fn message(&self) -> &str {
        "Mock"
    }

    fn details(&self) -> Option<&str> {
        None
    }

    fn hint(&self) -> Option<&str> {
        None
    }

    fn table_name(&self) -> Option<&str> {
        None
    }

    fn column_name(&self) -> Option<&str> {
        None
    }

    fn constraint_name(&self) -> Option<&str> {
        None
    }
}

/// A mock object that should have parity with the database schema and operations.
#[derive(Debug, Clone, Default)]
pub struct MockDatabase {
    pub(crate) users: Vec<User>,
    pub(crate) buckets: Vec<Bucket>,
    pub(crate) user_bucket_relations: Vec<BucketUserRelation>,
    pub(crate) questions: Vec<Question>,
    pub(crate) answers: Vec<Answer>,
    pub(crate) favorite_question_relations: Vec<FavoriteQuestionRelation>,
}

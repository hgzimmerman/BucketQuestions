//! A fixture for testing against configurations related to questions.
use crate::{
    bucket::db_types::{Bucket, NewBucket},
    bucket_user_relation::db_types::{BucketUserRelation, NewBucketUserRelation},
    question::db_types::{NewQuestion, Question},
    test::{fixture::Fixture, user_fixture::UserFixture},
    user::db_types::User,
    BoxedRepository,
};

/// Fixture that creates 2 users, 1 bucket, and one relation record in the repository.
/// user1 is joined to the bucket.
///
/// user has full permissions on bucket.
#[derive(Clone, Debug)]
pub struct QuestionFixture {
    /// Bucket
    pub bucket: Bucket,
    /// User
    pub user: User,
    /// Bucket user relation
    pub relation: BucketUserRelation,
    /// First question
    pub question1: Question,
    /// Second question
    pub question2: Question,
}

impl Fixture for QuestionFixture {
    fn generate(conn: &BoxedRepository) -> Self {
        let user = UserFixture::generate(conn).user;

        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string(),
        };
        let bucket = conn
            .create_bucket(new_bucket)
            .expect("Should be able to create bucket");

        let new_relation = NewBucketUserRelation {
            user_uuid: user.uuid,
            bucket_uuid: bucket.uuid,
            set_public_permission: true,
            set_drawing_permission: true,
            set_exclusive_permission: true,
            kick_permission: true,
            grant_permissions_permission: true,
        };

        let relation = conn
            .add_user_to_bucket(new_relation)
            .expect("Should be able to create user bucket relation");

        let mut new_question = NewQuestion {
            bucket_uuid: bucket.uuid,
            user_uuid: Some(user.uuid),
            question_text: "Is this the first question?".to_string(),
        };

        let question1 = conn
            .create_question(new_question.clone())
            .expect("Should create question");

        new_question.question_text = "Is this the second question?".to_string();
        let question2 = conn
            .create_question(new_question.clone())
            .expect("Should create question");

        QuestionFixture {
            bucket,
            user,
            relation,
            question1,
            question2,
        }
    }
}

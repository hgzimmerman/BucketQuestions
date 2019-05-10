use crate::Repository;
use diesel_reset::fixture::Fixture;
use crate::bucket::db_types::{NewBucket, Bucket, BucketUserRelation, NewBucketUserRelation, NewQuestion, Question, NewAnswer, Answer};
use crate::user::{NewUser, User};

/// Fixture that creates 2 users, 1 bucket, and one relation record in the repository.
/// user1 is joined to the bucket.
pub struct AnswerFixture {
    pub bucket: Bucket,
    pub user: User,
    pub relation: BucketUserRelation,
    pub question: Question,
    pub answer: Answer
}

impl Fixture for AnswerFixture
{
    type Repository = Box<dyn Repository>;

    fn generate(conn: &Box<Repository>) -> Self  {

        let new_user = NewUser {
            google_user_id: "123456789".to_string(),
            google_name: Some("Yeet".to_owned())
        };

        let user = conn.create_user(new_user).unwrap();

        let new_bucket = NewBucket {
            bucket_name: "bucket".to_string(),
            bucket_slug: "slug".to_string()
        };
        let bucket = conn.create_bucket(new_bucket).expect("Should be able to create bucket");

        let new_relation = NewBucketUserRelation {
            user_uuid: user.uuid,
            bucket_uuid: bucket.uuid,
            set_public_permission: true,
            set_drawing_permission: true,
            set_exclusive_permission: true,
            grant_permissions_permission: true
        };

        let relation = conn.add_user_to_bucket(new_relation).expect("Should be able to create user bucket relation");

        let new_question = NewQuestion {
            bucket_uuid: bucket.uuid,
            user_uuid: Some(user.uuid),
            question_text: "Is this the first question?".to_string()
        };

        let question = conn.create_question(new_question).expect("Should create question");

        let new_answer = NewAnswer {
            user_uuid: Some(user.uuid),
            question_uuid: question.uuid,
            publicly_visible: false,
            answer_text: "I think this is an answer".to_string()
        };
        let answer = conn.create_answer(new_answer).expect("Should create new answer");

        AnswerFixture {
            bucket,
            user,
            relation,
            question,
            answer
        }
    }
}
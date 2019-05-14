use crate::{
    question::db_types::{NewQuestion, Question},
    test::{bucket_fixture::BucketFixture, execute_test, question_fixture::QuestionFixture},
    BoxedRepository,
};

#[test]
fn create_question() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let new_question = NewQuestion {
            bucket_uuid: fixture.bucket.uuid,
            user_uuid: Some(fixture.user.uuid),
            question_text: "Another question! Cool?".to_string(),
        };

        db.create_question(new_question)
            .expect("Should be able to create question.");
    });
}

#[test]
fn create_question_without_user() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let new_question = NewQuestion {
            bucket_uuid: fixture.bucket.uuid,
            user_uuid: None,
            question_text: "Another question! Cool?".to_string(),
        };

        db.create_question(new_question)
            .expect("Should be able to create question.");
    });
}

#[test]
fn delete_question() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let question = db
            .delete_question(fixture.question1.uuid)
            .expect("Should be able to create question.");
        assert_eq!(question, fixture.question1);
    });
}

#[test]
fn get_random_question_single() {
    execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
        // Add one question to the bucket fixture config.
        let new_question = NewQuestion {
            bucket_uuid: fixture.bucket.uuid,
            user_uuid: None,
            question_text: "Another question! Cool?".to_string(),
        };

        let question = db
            .create_question(new_question)
            .expect("Should be able to create question.");

        let random_question: Option<Question> = db
            .get_random_question(fixture.bucket.uuid)
            .expect("should get random question");
        let random_question = random_question.expect("Should be one question in bucket");
        assert_eq!(question, random_question);
    })
}

#[test]
fn get_random_question_none() {
    execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
        let random_question: Option<Question> = db
            .get_random_question(fixture.bucket.uuid)
            .expect("should get random question");
        assert_eq!(random_question, None);
    });
}

#[test]
fn get_number_of_active_questions_for_bucket() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let num_questions = db
            .get_number_of_active_questions_for_bucket(fixture.bucket.uuid)
            .expect("Should get number of buckets");
        assert_eq!(num_questions, 2);
    });
}

#[test]
fn set_archived() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let question = db
            .set_archive_status_for_question(fixture.question1.uuid, true)
            .expect("Should set archived question");
        assert!(question.archived);
        let num_questions = db
            .get_number_of_active_questions_for_bucket(fixture.bucket.uuid)
            .expect("Should get number of questions");
        assert_eq!(num_questions, 1);

        let _question = db
            .set_archive_status_for_question(fixture.question2.uuid, true)
            .expect("Should set archived question");
        let num_questions = db
            .get_number_of_active_questions_for_bucket(fixture.bucket.uuid)
            .expect("Should get number of questions");
        assert_eq!(num_questions, 0);
    });
}

#[test]
fn get_questions_depending_on_achived_status() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let question = db
            .set_archive_status_for_question(fixture.question1.uuid, true)
            .expect("Should set archived buckets");
        assert!(question.archived);

        let archived_questions = db
            .get_all_questions_for_bucket_of_given_archived_status(fixture.bucket.uuid, true)
            .expect("Should get archived questions");
        assert_eq!(archived_questions.len(), 1);
        assert_eq!(archived_questions[0].uuid, fixture.question1.uuid);

        let active_questions = db
            .get_all_questions_for_bucket_of_given_archived_status(fixture.bucket.uuid, false)
            .expect("Should get active questions");
        assert_eq!(active_questions.len(), 1);
        assert_eq!(active_questions[0].uuid, fixture.question2.uuid);
    })
}

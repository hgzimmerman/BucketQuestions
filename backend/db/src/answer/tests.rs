use crate::{
    answer::db_types::{Answer, NewAnswer},
    test::{answer_fixture::AnswerFixture, execute_test},
    BoxedRepository,
};

#[test]
fn create_duplicate_answer() {
    // I guess you can create duplicate answers for now
    //    let (fixture, db) = setup::<AnswerFixture>();
    execute_test(|fixture: &AnswerFixture, db: BoxedRepository| {
        let new_answer = NewAnswer {
            user_uuid: Some(fixture.user.uuid),
            question_uuid: fixture.question.uuid,
            publicly_visible: false,
            answer_text: "I think this is an answer".to_string(),
        };
        let _answer: Answer = db
            .create_answer(new_answer)
            .expect("Should create new answer");
    });
}

#[test]
fn create_answer_without_answer() {
    execute_test(|fixture: &AnswerFixture, db: BoxedRepository| {
        let new_answer = NewAnswer {
            user_uuid: None,
            question_uuid: fixture.question.uuid,
            publicly_visible: false,
            answer_text: "I think this is an answer".to_string(),
        };
        let _answer: Answer = db
            .create_answer(new_answer)
            .expect("Should create new answer");
    })
}

#[test]
fn delete_answer() {
    execute_test(|fixture: &AnswerFixture, db: BoxedRepository| {
        let answer = db
            .delete_answer(fixture.answer.uuid)
            .expect("Should be able to delete answer");
        assert_eq!(answer, fixture.answer);
    })
}

#[test]
fn get_answers_for_question() {
    execute_test(|fixture: &AnswerFixture, db: BoxedRepository| {
        //    let (fixture, db) = setup::<AnswerFixture>();
        let answers = db
            .get_answers_for_question(fixture.question.uuid, true)
            .expect("Should get answers");
        assert_eq!(answers.len(), 0);

        let answers = db
            .get_answers_for_question(fixture.question.uuid, false)
            .expect("Should get all answers");
        assert_eq!(answers.len(), 1);
        assert_eq!(answers[0], fixture.answer);
    });
}

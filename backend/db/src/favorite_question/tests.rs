use crate::{
    favorite_question::db_types::NewFavoriteQuestionRelation,
    test::{question_fixture::QuestionFixture, util::execute_test},
    BoxedRepository,
};

#[test]
fn favorite_question() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let new_relation = NewFavoriteQuestionRelation {
            user_uuid: fixture.user.uuid,
            question_uuid: fixture.question1.uuid,
        };
        let _relation = db
            .favorite_question(new_relation)
            .expect("Should be able to favorite question");
    });
}

#[test]
fn unfavorite_question() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let new_relation = NewFavoriteQuestionRelation {
            user_uuid: fixture.user.uuid,
            question_uuid: fixture.question1.uuid,
        };
        let relation = db
            .favorite_question(new_relation)
            .expect("Should be able to favorite question");

        let delete_relation = db
            .unfavorite_question(new_relation)
            .expect("Should be able to unfavorite question");
        assert_eq!(relation, delete_relation)
    });
}

#[test]
fn get_favorite_questions() {
    execute_test(|fixture: &QuestionFixture, db: BoxedRepository| {
        let mut new_relation = NewFavoriteQuestionRelation {
            user_uuid: fixture.user.uuid,
            question_uuid: fixture.question1.uuid,
        };
        let _relation = db
            .favorite_question(new_relation)
            .expect("Should be able to favorite question");

        let favorites = db
            .get_favorite_questions(fixture.user.uuid)
            .expect("Sholud get favorite questions");
        assert_eq!(favorites.len(), 1);
        assert_eq!(favorites[0], fixture.question1);

        new_relation.question_uuid = fixture.question2.uuid;
        let _relation = db
            .favorite_question(new_relation)
            .expect("Should be able to favorite question");

        let favorites = db
            .get_favorite_questions(fixture.user.uuid)
            .expect("Sholud get favorite questions");
        assert_eq!(favorites.len(), 2);
        assert_eq!(favorites[0], fixture.question1); // Oldest - TODO should be other way around.
        assert_eq!(favorites[1], fixture.question2); // is Ordering guaranteed
                                                     // TODO order favorite questions by date
    });
}

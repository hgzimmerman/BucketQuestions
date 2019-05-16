use crate::{
    api::{answer::NewAnswerRequest, auth::test::get_jwt, routes},
    state::{test_util::execute_test_on_repository, State},
    util::test_util::deserialize,
};
use authorization::{AUTHORIZATION_HEADER_KEY, BEARER, Secret};
use db::{answer::db_types::Answer, test::answer_fixture::AnswerFixture, RepositoryProvider};
use warp::{http::StatusCode, test::request};

#[test]
fn answer_question_with_user() {
    execute_test_on_repository(|fix: &AnswerFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);
        let jwt = get_jwt(&state);

        let url = "/api/answer";

        let req = NewAnswerRequest {
            question_uuid: fix.question.uuid,
            publicly_visible: true,
            answer_text: "this is the answer".to_string(),
        };

        let resp = request()
            .method("POST")
            .json(&req)
            .header("content-length", "500")
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .path(url)
            .reply(&filter);

        assert_eq!(resp.status(), StatusCode::OK);

        let response = deserialize::<Answer>(&resp);
        assert!(response.publicly_visible);
        assert_eq!(response.answer_text, req.answer_text);
        assert_eq!(response.user_uuid, Some(fix.user.uuid));
    });
}

#[test]
fn answer_question_without_user() {
    execute_test_on_repository(|fix: &AnswerFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);

        let url = "/api/answer";

        let req = NewAnswerRequest {
            question_uuid: fix.question.uuid,
            publicly_visible: true,
            answer_text: "This is the answer".to_string(),
        };

        let resp = request()
            .method("POST")
            .json(&req)
            .header("content-length", "500")
            .path(url)
            .reply(&filter);

        assert_eq!(resp.status(), StatusCode::OK);

        let response = deserialize::<Answer>(&resp);
        assert!(response.publicly_visible);
        assert_eq!(response.answer_text, req.answer_text);
        assert_eq!(response.user_uuid, None);
    });
}

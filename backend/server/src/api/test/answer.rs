use crate::state::test_util::execute_test_on_repository;
use db::RepositoryProvider;
use crate::api::routes;
use crate::state::State;
use authorization::{Secret, AUTHORIZATION_HEADER_KEY, BEARER};
use warp::test::request;
use crate::util::test_util::deserialize;
use db::test::answer_fixture::AnswerFixture;
use crate::api::auth::test::get_jwt;
use crate::api::answer::NewAnswerRequest;
use db::bucket::db_types::Answer;
use warp::http::StatusCode;


#[test]
fn answer_question_with_user() {
    execute_test_on_repository(|fix: &AnswerFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new("hello"));
        let filter = routes(&state);
        let jwt = get_jwt(&state);

        let url = "/api/answer";

        let req = NewAnswerRequest {
            question_uuid: fix.question.uuid,
            publicly_visible: true,
            answer_text: "this is the answer".to_string()
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
        let state = State::testing_init(provider, Secret::new("hello"));
        let filter = routes(&state);

        let url = "/api/answer";

        let req = NewAnswerRequest {
            question_uuid: fix.question.uuid,
            publicly_visible: true,
            answer_text: "This is the answer".to_string()
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

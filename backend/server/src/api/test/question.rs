use crate::{
    api::{
        auth::test::get_jwt,
        question::{NewQuestionRequest, SetArchivedRequest},
        routes,
    },
    state::{test_util::execute_test_on_repository, State},
    util::test_util::deserialize,
};
use authorization::{AUTHORIZATION_HEADER_KEY, BEARER, Secret};
use db::{
    question::db_types::Question,
    test::{bucket_fixture::BucketFixture, question_fixture::QuestionFixture},
    RepositoryProvider,
};
use warp::{http::StatusCode, test::request};

#[test]
fn create_question_with_user_login() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);
        let jwt = get_jwt(&state);

        let url = "/api/question";

        let req = NewQuestionRequest {
            bucket_uuid: fix.bucket.uuid,
            question_text: "Are you still there?".to_string(),
        };

        let res = request()
            .method("POST")
            .json(&req)
            .header("content-length", "500")
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .path(url)
            .reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let res = deserialize::<Question>(&res);
        assert_eq!(res.bucket_uuid, req.bucket_uuid);
        assert_eq!(res.question_text, req.question_text);
        assert_eq!(res.user_uuid, Some(fix.user.uuid));
    });
}

#[test]
fn create_question_without_user_login() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);

        let url = "/api/question";

        let req = NewQuestionRequest {
            bucket_uuid: fix.bucket.uuid,
            question_text: "Are you still there?".to_string(),
        };

        let res = request()
            .method("POST")
            .json(&req)
            .header("content-length", "500")
            .path(url)
            .reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let res = deserialize::<Question>(&res);
        assert_eq!(res.bucket_uuid, req.bucket_uuid);
        assert_eq!(res.question_text, req.question_text);
        assert_eq!(res.user_uuid, None);
    });
}

#[test]
fn delete_question() {}

#[test]
fn random_question_populated() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);

        let url = format!("/api/question/random?bucket_uuid={}", fix.bucket.uuid);

        let res = request()
            .method("GET")
            .header("content-length", "500")
            .path(&url)
            .reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let res = deserialize::<Option<Question>>(&res);
        assert!(res.is_some());
    });
}

#[test]
fn random_question_unpopulated() {
    execute_test_on_repository(|fix: &BucketFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);

        let url = format!("/api/question/random?bucket_uuid={}", fix.bucket.uuid);

        let res = request().method("GET").path(&url).reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let res = deserialize::<Option<Question>>(&res);
        assert!(res.is_none());
    });
}

#[test]
fn num_questions_in_bucket() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);

        let url = format!("/api/question/number?bucket_uuid={}", fix.bucket.uuid);

        let res = request().method("GET").path(&url).reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let res = deserialize::<i64>(&res);
        assert_eq!(res, 2);
    });
}

#[test]
fn all_questions_in_bucket() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);

        let url = format!("/api/question/in_bucket?bucket_uuid={}", fix.bucket.uuid);

        let res = request().method("GET").path(&url).reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let res = deserialize::<Vec<Question>>(&res);
        assert_eq!(res.len(), 2);
    });
}

#[test]
fn all_questions_on_floor() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);

        let url = format!("/api/question/on_floor?bucket_uuid={}", fix.bucket.uuid);

        let res = request().method("GET").path(&url).reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let res = deserialize::<Vec<Question>>(&res);
        assert_eq!(res.len(), 0);
    });
}

#[test]
fn set_question_archived_state() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);

        let url = "/api/question/archive";

        let req = SetArchivedRequest {
            question_uuid: fix.question1.uuid,
            archived: true,
        };

        let res = request()
            .method("PUT")
            .json(&req)
            .header("content-length", "500")
            .path(&url)
            .reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let res = deserialize::<Question>(&res);
        assert!(res.archived);
    });
}

#[test]
fn favorite_question() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);
        let jwt = get_jwt(&state);

        let url = format!("/api/question/{}/favorite", fix.question1.uuid);

        let res = request()
            .method("POST")
            .header("content-length", "500")
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .path(&url)
            .reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let _res = deserialize::<()>(&res);
    });
}

#[test]
fn unfavorite_question() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);
        let jwt = get_jwt(&state);

        let url = format!("/api/question/{}/favorite", fix.question1.uuid);

        let res = request()
            .method("POST")
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .path(&url)
            .reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let _res = deserialize::<()>(&res);

        let res = request()
            .method("DELETE")
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .path(&url)
            .reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let _res = deserialize::<()>(&res);
    });
}

#[test]
fn get_favorite_questions() {
    execute_test_on_repository(|fix: &QuestionFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);
        let jwt = get_jwt(&state);

        let url = format!("/api/question/{}/favorite", fix.question1.uuid);

        let res = request()
            .method("POST")
            .header("content-length", "500")
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .path(&url)
            .reply(&filter);

        assert_eq!(res.status(), StatusCode::OK);

        let _res = deserialize::<()>(&res);

        let url = "/api/question/favorites";
        let res = request()
            .method("GET")
            .header("content-length", "500")
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .path(&url)
            .reply(&filter);

        let res = deserialize::<Vec<Question>>(&res);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].uuid, fix.question1.uuid);
    });
}

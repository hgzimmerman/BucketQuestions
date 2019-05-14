use crate::state::test_util::execute_test_on_repository;
use db::test::empty_fixture::EmptyFixture;
use db::RepositoryProvider;
use crate::api::routes;
use crate::state::State;
use authorization::Secret;
use warp::test::request;
use crate::api::auth::LinkResponse;
use crate::util::test_util::deserialize;

#[test]
fn get_auth_link() {
    execute_test_on_repository(|_fix: &EmptyFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new("hello"));
        let filter = routes(&state);

        let resp = request()
            .method("GET")
            .path("/api/auth/link")
            .reply(&filter);

        let _ = deserialize::<LinkResponse>(&resp);
    });
}
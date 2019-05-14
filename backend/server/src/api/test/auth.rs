use crate::{
    api::{auth::LinkResponse, routes},
    state::{test_util::execute_test_on_repository, State},
    util::test_util::deserialize,
};
use authorization::Secret;
use db::{test::empty_fixture::EmptyFixture, RepositoryProvider};
use warp::test::request;

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

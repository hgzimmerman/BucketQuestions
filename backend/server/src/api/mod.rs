//! The api defines all of the routes that are supported for the server.
mod answer;
mod auth;
mod bucket;
mod question;
mod user;

use warp::Reply;

use warp::{path, Filter};

use crate::{
    api::{
        answer::answer_api, auth::auth_api, bucket::bucket_api, question::question_api,
        user::user_api,
    },
    state::State,
    static_files::{static_files_handler, FileConfig},
};
use warp::{filters::BoxedFilter, Rejection};

/// The initial segment in the uri path.
pub const API_STRING: &str = "api";

/// The core of the exposed routes.
/// Anything that sits behind this filter accesses the DB in some way.
pub fn api(state: &State) -> BoxedFilter<(impl Reply,)> {
    //impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path(API_STRING)
        .and(
            bucket_api(state)
                .or(answer_api(state))
                .or(question_api(state))
                .or(auth_api(state))
                .or(user_api(state)),
        )
        .boxed()
}

/// A filter that is responsible for configuring everything that can be served.
///
/// # Notes
/// It is responsible for:
/// * Routes the API
/// * Handles file requests and redirections
/// * Initializes warp logging
/// * converts errors
/// * Handles CORS
pub fn routes(state: &State) -> impl Filter<Extract = (impl Reply,), Error = Rejection> {
    let cors = warp::cors()
        //        .allow_origin("http://localhost:8081")
        .allow_headers(vec![
            "Access-Control-Allow-Origin",
            "content-type",
            "Authorization",
        ])
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    let file_config = FileConfig::new(state.server_lib_root());

    api(state)
        .or(static_files_handler(file_config))
        .with(warp::log("routes"))
        .with(cors)
        .recover(crate::error::customize_error)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{state::State};
    use db::test::empty_fixture::EmptyFixture;
    use authorization::Secret;
    use crate::state::test_util::execute_test_on_repository;
    use warp::test::request;
    use crate::util::test_util::deserialize;
    use crate::api::auth::LinkResponse;
    use db::RepositoryProvider;

    #[test]
    fn get_auth_link() {
        execute_test_on_repository(|_fix: &EmptyFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new("hello"));
            let filter = routes(&state);

            let resp = request()
                .method("GET")
                .path("/api/auth/link")
                .reply(&filter);

            let _ = deserialize::<LinkResponse>(resp);
        });

   }
}

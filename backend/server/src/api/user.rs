use crate::state::State;
use warp::{filters::BoxedFilter, path, Filter, Reply};

pub const USER_PATH: &str = "user";
pub fn user_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    // impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    path(USER_PATH).map(|| "unimplemented").boxed()
}

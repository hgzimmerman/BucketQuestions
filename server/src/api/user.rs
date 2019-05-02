use crate::state::State;
use warp::{Filter, Reply, Rejection};
use warp::path;

pub const USER_PATH: &str = "user";
pub fn user_api(state: &State) -> impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    path(USER_PATH)
        .map(||"unimplemented")
}
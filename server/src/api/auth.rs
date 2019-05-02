use crate::state::State;
use warp::{Filter, Reply, Rejection};
use warp::path;

pub const AUTH_PATH: &str = "auth";
pub fn auth_api(state: &State) -> impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    path(AUTH_PATH)
        .map(||"unimplemented")
}
use crate::state::State;
use warp::{Filter, Reply, Rejection};
use warp::path;

pub const ANSWER_PATH: &str = "answer";
pub fn answer_api(state: &State) -> impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    path(ANSWER_PATH)
        .map(||"unimplemented")
}
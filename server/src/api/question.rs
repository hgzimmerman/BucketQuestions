use crate::state::State;
use warp::{Filter, Reply, Rejection};
use warp::path;

pub const QUESTION_PATH: &str = "question";
pub fn question_api(state: &State) -> impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    path(QUESTION_PATH)
        .map(||"unimplemented")
}
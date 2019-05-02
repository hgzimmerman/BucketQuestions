use crate::state::State;
use warp::{Filter, Reply, Rejection};
use warp::path;

pub const BUCKET_PATH: &str = "bucket";
pub fn bucket_api(state: &State) -> impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{

    path(BUCKET_PATH)
        .map(||"unimplemented")
}
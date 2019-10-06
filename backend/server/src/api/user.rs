use crate::error::Error;
use crate::server_auth::user_filter;
use crate::state::State;
use crate::util::json_or_reject;
use db::user::db_types::User;
use db::BoxedRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{filters::BoxedFilter, path, Filter, Reply};

/// A response containing just a uuid.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UuidResponse {
    uuid: Uuid,
}

pub const USER_PATH: &str = "user";
pub fn user_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    // Gets the user
    let get_user = warp::path::end()
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db())
        .map(
            |user_uuid: Uuid, db: BoxedRepository| -> Result<User, Error> {
                db.get_user(user_uuid).map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    // Gets extracts the uuid from the user and returns it to them.
    let get_uuid = path!("uuid")
        .and(warp::path::end())
        .and(user_filter(state))
        .map(|uuid: Uuid| warp::reply::json(&uuid));

    path(USER_PATH).and(get_user.or(get_uuid)).boxed()
}

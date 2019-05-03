use crate::state::State;
use warp::{Filter, Reply};
use warp::path;
use crate::server_auth::user_filter;
use crate::util::{json_body_filter, json_or_reject};
use pool::{PooledConn};
use db::bucket::interface::{BucketRepository, BucketUserRelationRepository};
use uuid::Uuid;
use db::bucket::db_types::{NewBucket, Bucket, BucketUserJoin, NewBucketUserJoin, BucketUserPermissionsChangeset, BucketUserPermissions};
use crate::error::Error;
use serde::{Serialize, Deserialize};
use db::user::User;
use warp::filters::BoxedFilter;


pub const BUCKET_PATH: &str = "bucket";


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetPermissionsRequest {
    pub target_user_uuid: Uuid,
    /// Can the user set the visibility of the bucket.
    pub set_visibility_permission: Option<bool>,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: Option<bool>,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: Option<bool>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeDrawingRequest {
    drawing: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeVisibilityRequest {
    visible: bool
}


pub fn bucket_api(state: &State) -> BoxedFilter<(impl Reply,)> { //impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{
    // Returning a boxed filter improves compile times significantly


    // Must be logged in to create a bucket
    let create_bucket = warp::post2()
        .and(warp::path::end())
        .and(json_body_filter(2))
        .and(user_filter(state))
        .and(state.db())
        .map(|request: NewBucket, user_uuid: Uuid, conn: PooledConn| -> Result<Bucket, Error> {
            let bucket = conn.create_bucket(request)?;
            let new_relation = NewBucketUserJoin {
                user_uuid,
                bucket_uuid: bucket.uuid,
                set_visibility_permission: true,
                set_drawing_permission: true,
                grant_permissions_permission: true
            };
            conn.add_user_to_bucket(new_relation)?;
            Ok(bucket)
        })
        .and_then(json_or_reject);

    let get_bucket = path!(String)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db())
        .map(|slug: String, conn: PooledConn| -> Result<Bucket, Error> {
           conn.get_bucket_by_slug(slug).map_err(Error::from)
        })
        .and_then(json_or_reject);

    let get_bucket_by_uuid = path!(Uuid)
        .and(warp::get2())
        .and(state.db())
        .map(get_bucket_by_uuid_handler)
        .and_then(json_or_reject);

    let get_buckets_user_is_in = path!("in")
        .and(warp::path::end())
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db())
        .map(get_buckets_user_is_in_handler)
        .and_then(json_or_reject);

    let get_public_buckets = path!("public")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db())
        .map(get_public_buckets_handler)
        .and_then(json_or_reject);

    // User is joining the bucket themselves
    let add_self_to_bucket = path!(Uuid / "user")
        .and(warp::path::end())
        .and(warp::post2())
        .and(user_filter(state))
        .and(state.db())
        .map(add_self_to_bucket_handler)
        .and_then(json_or_reject);

    // Gets permissions for a bucket for the logged in user.
    let get_permissions_for_self = path!( Uuid / "user")
        .and(warp::path::end())
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db())
        .map(get_permissions_for_self_handler)
        .and_then(json_or_reject);

    let set_permissions = path!(Uuid / "user")
        .and(warp::path::end())
        .and(warp::put2())
        .and(json_body_filter(2))
        .and(user_filter(state))
        .and(state.db())
        .map(set_permissions_handler)
        .and_then(json_or_reject);

    let set_bucket_drawing = path!(Uuid)
        .and(warp::path::end())
        .and(warp::put2())
        .and(json_body_filter(1))
        .and(user_filter(state))
        .and(state.db())
        .map(set_bucket_drawing_handler)
        .and_then(json_or_reject);

    let set_bucket_visibility = path!(Uuid)
        .and(warp::path::end())
        .and(warp::put2())
        .and(json_body_filter(1))
        .and(user_filter(state))
        .and(state.db())
        .map(set_bucket_visibility_handler)
        .and_then(json_or_reject);

    let get_users_in_bucket = path!(Uuid / "users")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db())
        .map(get_users_in_bucket_handler)
        .and_then(json_or_reject);

    path(BUCKET_PATH)
        .and(
            create_bucket
                .or(get_bucket_by_uuid)
                .or(get_buckets_user_is_in)
                .or(get_public_buckets)
                .or(add_self_to_bucket)
                .or(get_permissions_for_self)
                .or(set_permissions)
                .or(set_bucket_drawing)
                .or(set_bucket_visibility)
                .or(get_users_in_bucket)
                .or(get_bucket) // This should be near the end to avoid slugs matching before other significant paths
        )
        .boxed()
}

/// Adds a user to the bucket.
/// This user has no permissions by default.
fn add_self_to_bucket_handler(bucket_uuid: Uuid, user_uuid: Uuid, conn: PooledConn) -> Result<BucketUserJoin, Error> {
    let new_relation = NewBucketUserJoin {
        user_uuid,
        bucket_uuid,
        set_visibility_permission: false,
        set_drawing_permission: false,
        grant_permissions_permission: false
    };
    conn.add_user_to_bucket(new_relation).map_err(Error::from)
}

fn set_bucket_drawing_handler(bucket_uuid: Uuid, request: ChangeDrawingRequest, user_uuid: Uuid, conn: PooledConn) -> Result<Bucket, Error> {
    let permissions_for_acting_user = conn.get_permissions(user_uuid, bucket_uuid).map_err(Error::from)?;
    if permissions_for_acting_user.set_drawing_permission {
        conn.change_drawing_status(bucket_uuid, request.drawing).map_err(Error::from)
    } else {
        Err(Error::not_authorized("User does not have privileges to change the drawing status of the bucket."))
    }
}

fn set_bucket_visibility_handler(bucket_uuid: Uuid, request: ChangeVisibilityRequest, user_uuid: Uuid, conn: PooledConn) -> Result<Bucket, Error> {
    let permissions_for_acting_user = conn.get_permissions(user_uuid, bucket_uuid).map_err(Error::from)?;
    if permissions_for_acting_user.set_drawing_permission {
        conn.change_visibility(bucket_uuid, request.visible).map_err(Error::from)
    } else {
        Err(Error::not_authorized("User does not have privileges to change the visibility status of the bucket."))
    }
}

fn get_users_in_bucket_handler(bucket_uuid: Uuid, conn: PooledConn) -> Result<Vec<User>, Error> {
    conn.get_users_in_bucket(bucket_uuid).map_err(Error::from)
}

fn set_permissions_handler(bucket_uuid: Uuid, permissions_request: SetPermissionsRequest, user_uuid: Uuid, conn: PooledConn) -> Result<BucketUserJoin, Error> {
    let permissions_for_acting_user = conn.get_permissions(user_uuid, bucket_uuid).map_err(Error::from)?;
    if permissions_for_acting_user.grant_permissions_permission {
        // The permissions of the target user
        let current_user_permissions = conn.get_permissions(permissions_request.target_user_uuid, bucket_uuid).map_err(Error::from)?;
        let permissions_changeset = BucketUserPermissionsChangeset {
            uuid: current_user_permissions.uuid,
            set_visibility_permission: permissions_request.set_visibility_permission,
            set_drawing_permission: permissions_request.set_drawing_permission,
            grant_permissions_permission: permissions_request.grant_permissions_permission
        };
        conn.set_permissions(permissions_changeset).map_err(Error::from)
    } else {
        Err(Error::not_authorized("User does not have privileges to set permissions for other users."))
    }
}

fn get_permissions_for_self_handler(bucket_uuid: Uuid, user_uuid: Uuid, conn: PooledConn) -> Result<BucketUserPermissions, Error> {
    conn.get_permissions(user_uuid, bucket_uuid).map_err(Error::from)
}


fn get_public_buckets_handler(conn: PooledConn) -> Result<Vec<Bucket>, Error> {
    conn.get_publicly_visible_buckets().map_err(Error::from)
}

fn get_buckets_user_is_in_handler(user_uuid: Uuid, conn: PooledConn) -> Result<Vec<Bucket>, Error> {
    conn.get_buckets_user_is_a_part_of(user_uuid).map_err(Error::from)
}

fn get_bucket_by_uuid_handler(uuid: Uuid, conn: PooledConn)-> Result<Bucket, Error> {
   conn.get_bucket_by_uuid(uuid).map_err(Error::from)
}

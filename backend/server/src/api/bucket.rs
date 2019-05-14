use crate::{
    error::Error,
    server_auth::user_filter,
    state::State,
    util::{json_body_filter, json_or_reject},
};
use db::{bucket::{
    db_types::{
        Bucket, BucketFlagChangeset, BucketUserRelation, BucketUserPermissions,
        BucketUserPermissionsChangeset, NewBucket, NewBucketUserRelation,
    },
}, user::db_types::User, BoxedRepository};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{filters::BoxedFilter, path, Filter, Reply};

pub const BUCKET_PATH: &str = "bucket";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetPermissionsRequest {
    pub target_user_uuid: Uuid,
    /// Can the user set the visibility of the bucket.
    pub set_visibility_permission: Option<bool>,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: Option<bool>,
    /// Can the user set the bucket to private.
    pub set_private_permission: Option<bool>,
    /// Can the user grant permissions to other users.
    pub grant_permissions_permission: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeDrawingRequest {
    drawing: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeVisibilityRequest {
    visible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeBucketFlagsRequest {
    /// Is the bucket visible
    pub visible: Option<bool>,
    /// Is the bucket session currently active.
    pub drawing_enabled: Option<bool>,
    /// Can an unjoined user join the bucket.
    pub private: Option<bool>,
}

pub fn bucket_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    //impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{
    // Returning a boxed filter improves compile times significantly

    // Must be logged in to create a bucket
    let create_bucket = warp::post2()
        .and(warp::path::end())
        .and(json_body_filter(2))
        .and(user_filter(state))
        .and(state.db2())
        .map(create_bucket_handler)
        .and_then(json_or_reject);

    let get_bucket = path!("slug" / String)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db2())
        .map(|slug: String, conn: BoxedRepository| -> Result<Bucket, Error> {
            info!("get_bucket");
            conn.get_bucket_by_slug(slug).map_err(Error::from)
        })
        .and_then(json_or_reject);

    let get_bucket_by_uuid = path!(Uuid)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db2())
        .map(get_bucket_by_uuid_handler)
        .and_then(json_or_reject);

    let get_buckets_user_is_in = path("in")
        .and(warp::path::end())
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db2())
        .map(get_buckets_user_is_in_handler)
        .and_then(json_or_reject);

    let get_public_buckets = path!("public")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db2())
        .map(get_public_buckets_handler)
        .and_then(json_or_reject);

    // User is joining the bucket themselves
    let add_self_to_bucket = path!(Uuid / "user")
        .and(warp::path::end())
        .and(warp::post2())
        .and(user_filter(state))
        .and(state.db2())
        .map(add_self_to_bucket_handler)
        .and_then(json_or_reject);

    // Gets permissions for a bucket for the logged in user.
    let get_permissions_for_self = path!(Uuid / "user")
        .and(warp::path::end())
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db2())
        .map(get_permissions_for_self_handler)
        .and_then(json_or_reject);

    let set_permissions = path!(Uuid / "user")
        .and(warp::path::end())
        .and(warp::put2())
        .and(json_body_filter(2))
        .and(user_filter(state))
        .and(state.db2())
        .map(set_permissions_handler)
        .and_then(json_or_reject);

    let set_bucket_flags = path!(Uuid)
        .and(warp::path::end())
        .and(warp::put2())
        .and(json_body_filter(1))
        .and(user_filter(state))
        .and(state.db2())
        .map(set_bucket_flags_handler)
        .and_then(json_or_reject);

    let get_users_in_bucket = path!(Uuid / "users")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db2())
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
                .or(set_bucket_flags)
                .or(get_users_in_bucket)
                .or(get_bucket),
        )
        .boxed()
}


fn create_bucket_handler(request: NewBucket, user_uuid: Uuid, conn: BoxedRepository) -> Result<Bucket, Error> {
    info!("add_self_to_bucket_handler");
    let bucket = conn.create_bucket(request)?;
    let new_relation = NewBucketUserRelation {
        user_uuid,
        bucket_uuid: bucket.uuid,
        set_public_permission: true,
        set_drawing_permission: true,
        set_exclusive_permission: true,
        grant_permissions_permission: true,
    };
    conn.add_user_to_bucket(new_relation)?;
    Ok(bucket)
}

/// Adds a user to the bucket.
/// This user has no permissions by default.
fn add_self_to_bucket_handler(
    bucket_uuid: Uuid,
    user_uuid: Uuid,
    conn: BoxedRepository,
) -> Result<BucketUserRelation, Error> {
    info!("add_self_to_bucket_handler");
    let new_relation = NewBucketUserRelation {
        user_uuid,
        bucket_uuid,
        set_public_permission: false,
        set_drawing_permission: false,
        set_exclusive_permission: false,
        grant_permissions_permission: false,
    };
    conn.add_user_to_bucket(new_relation).map_err(Error::from)
}

/// Won't reject a request, but will just drop requests to change settings that the user isn't authorized to do.
fn set_bucket_flags_handler(
    bucket_uuid: Uuid,
    request: ChangeBucketFlagsRequest,
    user_uuid: Uuid,
    conn: BoxedRepository,
) -> Result<Bucket, Error> {
    info!("set_bucket_flags_handler");
    let permissions_for_acting_user = conn
        .get_permissions(user_uuid, bucket_uuid)
        .map_err(Error::from)?;
    fn verify_permission(permission: bool, flag: Option<bool>) -> Option<bool> {
        if permission {
            flag
        } else {
            None
        }
    }
    let changeset = BucketFlagChangeset {
        uuid: bucket_uuid,
        public_viewable: verify_permission(
            permissions_for_acting_user.set_public_permission,
            request.visible,
        ),
        drawing_enabled: verify_permission(
            permissions_for_acting_user.set_drawing_permission,
            request.drawing_enabled,
        ),
        exclusive: verify_permission(
            permissions_for_acting_user.set_exclusive_permission,
            request.drawing_enabled,
        ),
    };
    conn.change_bucket_flags(changeset).map_err(Error::from)
}

fn get_users_in_bucket_handler(bucket_uuid: Uuid, conn: BoxedRepository) -> Result<Vec<User>, Error> {
    info!("get_users_in_bucket_handler");
    conn.get_users_in_bucket(bucket_uuid).map_err(Error::from)
}

fn set_permissions_handler(
    bucket_uuid: Uuid,
    permissions_request: SetPermissionsRequest,
    user_uuid: Uuid,
    conn: BoxedRepository,
) -> Result<BucketUserRelation, Error> {
    info!("set_permissions_handler");
    let permissions_for_acting_user = conn
        .get_permissions(user_uuid, bucket_uuid)
        .map_err(Error::from)?;
    if permissions_for_acting_user.grant_permissions_permission {
        let permissions_changeset = BucketUserPermissionsChangeset {
            user_uuid,
            bucket_uuid,
            set_public_permission: permissions_request.set_visibility_permission,
            set_drawing_permission: permissions_request.set_drawing_permission,
            set_exclusive_permission: permissions_request.set_private_permission,
            grant_permissions_permission: permissions_request.grant_permissions_permission,
        };
        conn.set_permissions(permissions_changeset)
            .map_err(Error::from)
    } else {
        Err(Error::not_authorized(
            "User does not have privileges to set permissions for other users.",
        ))
    }
}

fn get_permissions_for_self_handler(
    bucket_uuid: Uuid,
    user_uuid: Uuid,
    conn: BoxedRepository,
) -> Result<BucketUserPermissions, Error> {
    info!("get_permissions_for_self_handler");
    conn.get_permissions(user_uuid, bucket_uuid)
        .map_err(Error::from)
}

fn get_public_buckets_handler(conn: BoxedRepository) -> Result<Vec<Bucket>, Error> {
    info!("get_public_buckets_handler");
    conn.get_publicly_visible_buckets().map_err(Error::from)
}

fn get_buckets_user_is_in_handler(user_uuid: Uuid, conn: BoxedRepository) -> Result<Vec<Bucket>, Error> {
    info!("get_buckets_user_is_in_handler");
    conn.get_buckets_user_is_a_part_of(user_uuid)
        .map_err(Error::from)
}

fn get_bucket_by_uuid_handler(uuid: Uuid, conn: BoxedRepository) -> Result<Bucket, Error> {
    info!("get_bucket_by_uuid_handler");
    conn.get_bucket_by_uuid(uuid).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::test::setup;
    use db::test::bucket_fixture::BucketFixture;
    use db::user::db_types::NewUser;
    use db::test::user_fixture::UserFixture;
    use db::test::bucket_user_relation_fixture::UserBucketRelationFixture;


    #[test]
    fn add_self_to_bucket() {
        let (fixture, db) = setup::<BucketFixture>();

        let new_user = NewUser {
            google_user_id: "12".to_string(),
            google_name: None
        };
        let user = db.create_user(new_user).expect("Should create new user");

        let relation = add_self_to_bucket_handler(fixture.bucket.uuid, user.uuid, db)
            .expect("Should add user to bucket");
        assert!(!relation.grant_permissions_permission);
        assert!(!relation.set_public_permission);
        assert!(!relation.set_exclusive_permission);
        assert!(!relation.set_drawing_permission);
    }

    #[test]
    fn set_bucket_flags() {
        let (fixture, db) = setup::<UserBucketRelationFixture>();

        let request = ChangeBucketFlagsRequest {
            visible: None,
            drawing_enabled: None,
            private: None
        };

        let bucket = set_bucket_flags_handler(fixture.bucket.uuid, request, fixture.user1.uuid, db)
            .expect("Bucket should be returned after changing flags.");
        assert_eq!(fixture.bucket, bucket);
    }

    #[test]
    fn create_bucket() {
        let (fixture, db) = setup::<UserFixture>();
        let bucket_name = "Bucket".to_string();
        let bucket_slug = "bucket".to_string();
        let new_bucket = NewBucket {
            bucket_name: bucket_name.clone(),
            bucket_slug: bucket_slug.clone()
        };
        let bucket = create_bucket_handler(new_bucket, fixture.user.uuid, db).expect("Should create bucket");
        assert_eq!(bucket.bucket_name, bucket_name);
        assert_eq!(bucket.bucket_slug, bucket_slug);
    }

}

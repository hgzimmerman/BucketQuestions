use crate::{
    error::Error,
    server_auth::user_filter,
    state::State,
    util::{json_or_reject, sized_body_json},
};
use db::{
    bucket::db_types::{Bucket, BucketFlagChangeset, NewBucket},
    bucket_user_relation::db_types::{
        BucketUserPermissions, BucketUserPermissionsChangeset, BucketUserRelation,
        NewBucketUserRelation,
    },
    user::db_types::User,
    BoxedRepository,
};
use diesel::result::DatabaseErrorKind;
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{filters::BoxedFilter, path, Filter, Reply};

pub const BUCKET_PATH: &str = "bucket";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserUuidQueryParam {
    pub user_uuid: Uuid
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetPermissionsRequest {
    pub target_user_uuid: Uuid,
    /// Can the user set the visibility of the bucket.
    pub set_public_permission: Option<bool>,
    /// Can the user enable drawing from the bucket.
    pub set_drawing_permission: Option<bool>,
    /// Can the user set the bucket to private.
    pub set_exclusive_permission: Option<bool>,
    /// Can the user kick other users.
    pub kick_permission: Option<bool>,
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
    pub publicly_visible: Option<bool>,
    /// Is the bucket session currently active.
    pub drawing_enabled: Option<bool>,
    /// Can an unjoined user join the bucket.
    pub exclusive: Option<bool>,
}

/// Request to create a bucket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBucketRequest {
    pub bucket_name: String,
}

pub fn bucket_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    //impl Filter<Extract=(impl Reply,), Error=Rejection> + Clone{
    // Returning a boxed filter improves compile times significantly

    // Must be logged in to create a bucket
    let create_bucket = warp::post2()
        .and(warp::path::end())
        .and(sized_body_json(2))
        .and(user_filter(state))
        .and(state.db())
        .map(create_bucket_handler)
        .and_then(json_or_reject);

    let get_bucket = path!("slug" / String)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db())
        .map(
            |slug: String, conn: BoxedRepository| -> Result<Bucket, Error> {
                info!("get_bucket");
                conn.get_bucket_by_slug(slug).map_err(Error::from)
            },
        )
        .and_then(json_or_reject);

    let get_bucket_by_uuid = path!(Uuid)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.db())
        .map(get_bucket_by_uuid_handler)
        .and_then(json_or_reject);

    let get_buckets_user_is_in = path("in")
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

    let remove_user_from_bucket = path!(Uuid / "user")
        .and(warp::path::end())
        .and(warp::delete2())
        .and(user_filter(state))
        .and(warp::query())
        .and(state.db())
        .map(remove_user_from_bucket_handler)
        .and_then(json_or_reject);

    // Gets permissions for a bucket for the logged in user.
    let get_permissions_for_self = path!(Uuid / "user")
        .and(warp::path::end())
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db())
        .map(get_permissions_for_self_handler)
        .and_then(json_or_reject);

    let set_permissions = path!(Uuid / "user")
        .and(warp::path::end())
        .and(warp::put2())
        .and(sized_body_json(2))
        .and(user_filter(state))
        .and(state.db())
        .map(set_permissions_handler)
        .and_then(json_or_reject);

    let set_bucket_flags = path!(Uuid)
        .and(warp::path::end())
        .and(warp::put2())
        .and(sized_body_json(1))
        .and(user_filter(state))
        .and(state.db())
        .map(set_bucket_flags_handler)
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
                .or(remove_user_from_bucket)
                .or(get_permissions_for_self)
                .or(set_permissions)
                .or(set_bucket_flags)
                .or(get_users_in_bucket)
                .or(get_bucket),
        )
        .boxed()
}

fn create_bucket_handler(
    request: NewBucketRequest,
    user_uuid: Uuid,
    conn: BoxedRepository,
) -> Result<Bucket, Error> {
    info!("add_self_to_bucket_handler");

    fn bucket_already_exists(
        slug: &String,
        conn: &BoxedRepository,
    ) -> Result<bool, diesel::result::Error> {
        conn.get_bucket_by_slug(slug.clone())
            .map(|_| true)
            .or_else(|e| {
                if let diesel::result::Error::NotFound = e {
                    Ok(false)
                } else {
                    Err(e)
                }
            })
    }
    let slug = slug::slugify(&request.bucket_name);
    let mut candidate_slug = slug.clone();
    let mut id = 0;

    while bucket_already_exists(&candidate_slug, &conn)? {
        candidate_slug = format!("{}-{}", slug, id);
        id += 1;
    }

    let new_bucket = NewBucket {
        bucket_name: request.bucket_name,
        bucket_slug: candidate_slug,
    };

    let bucket = conn.create_bucket(new_bucket)?;
    // By default, the user has full permissions when they create a bucket.
    let new_relation = NewBucketUserRelation {
        user_uuid,
        bucket_uuid: bucket.uuid,
        set_public_permission: true,
        set_drawing_permission: true,
        set_exclusive_permission: true,
        kick_permission: true,
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

    let bucket = conn.get_bucket_by_uuid(bucket_uuid)?;
    if bucket.exclusive {
        return Err(Error::PreconditionNotMet("Bucket is set to exclusive. Users are not allowed to join.".to_string()));
    }

    info!("add_self_to_bucket_handler");
    // By default, users don't have any permissions.
    let new_relation = NewBucketUserRelation {
        user_uuid,
        bucket_uuid,
        set_public_permission: false,
        set_drawing_permission: false,
        set_exclusive_permission: false,
        kick_permission: false,
        grant_permissions_permission: false,
    };
    conn.add_user_to_bucket(new_relation).map_err(|e| {
        if let diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = e {
            Error::PreconditionNotMet(
                "There is already a relation between this user and the bucket".to_string(),
            )
        } else {
            Error::from(e)
        }
    })
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
            request.publicly_visible,
        ),
        drawing_enabled: verify_permission(
            permissions_for_acting_user.set_drawing_permission,
            request.drawing_enabled,
        ),
        exclusive: verify_permission(
            permissions_for_acting_user.set_exclusive_permission,
            request.exclusive,
        ),
    };
    conn.change_bucket_flags(changeset).map_err(Error::from)
}


fn remove_user_from_bucket_handler(bucket_uuid: Uuid, account_user_uuid: Uuid, target_user_uuid: UserUuidQueryParam, db: BoxedRepository) -> Result<BucketUserRelation, Error> {
    info!("remove_user_from_bucket_handler");

    let relation = db.get_user_bucket_relation(account_user_uuid, bucket_uuid)?;


    // Does user have permission to remove user
    if relation.kick_permission || account_user_uuid == target_user_uuid.user_uuid{
        db.remove_user_from_bucket(target_user_uuid.user_uuid, bucket_uuid).map_err(Error::from)
    } else {
        Err(Error::PreconditionNotMet("User does not have permission to remove another user from this bucket.".to_string()))
    }
}

fn get_users_in_bucket_handler(
    bucket_uuid: Uuid,
    conn: BoxedRepository,
) -> Result<Vec<User>, Error> {
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
            set_public_permission: permissions_request.set_public_permission,
            set_drawing_permission: permissions_request.set_drawing_permission,
            set_exclusive_permission: permissions_request.set_exclusive_permission,
            kick_permission: permissions_request.kick_permission,
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

fn get_buckets_user_is_in_handler(
    user_uuid: Uuid,
    conn: BoxedRepository,
) -> Result<Vec<Bucket>, Error> {
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
    use db::{
        test::{
            bucket_fixture::BucketFixture, bucket_user_relation_fixture::UserBucketRelationFixture,
            user_fixture::UserFixture, util::execute_test,
        },
        user::db_types::NewUser,
    };

    #[test]
    fn add_self_to_bucket() {
        execute_test(|fixture: &BucketFixture, db: BoxedRepository| {
            let new_user = NewUser {
                google_user_id: "12".to_string(),
                google_name: None,
            };
            let user = db.create_user(new_user).expect("Should create new user");

            let relation = add_self_to_bucket_handler(fixture.bucket.uuid, user.uuid, db)
                .expect("Should add user to bucket");
            assert!(!relation.grant_permissions_permission);
            assert!(!relation.set_public_permission);
            assert!(!relation.set_exclusive_permission);
            assert!(!relation.set_drawing_permission);
        });
    }

    #[test]
    fn set_bucket_flags() {
        execute_test(|fixture: &UserBucketRelationFixture, db: BoxedRepository| {
            let request = ChangeBucketFlagsRequest {
                publicly_visible: None,
                drawing_enabled: None,
                exclusive: None,
            };

            let bucket =
                set_bucket_flags_handler(fixture.bucket.uuid, request, fixture.user1.uuid, db)
                    .expect("Bucket should be returned after changing flags.");
            assert_eq!(fixture.bucket, bucket);
        });
    }

    #[test]
    fn create_bucket() {
        execute_test(|fixture: &UserFixture, db: BoxedRepository| {
            let bucket_name = "Bucket".to_string();
            let new_bucket = NewBucketRequest {
                bucket_name: bucket_name.clone(),
            };
            let expected_slug = "bucket".to_string();

            let bucket = create_bucket_handler(new_bucket, fixture.user.uuid, db)
                .expect("Should create bucket");
            assert_eq!(bucket.bucket_name, bucket_name);
            assert_eq!(bucket.bucket_slug, expected_slug);
        })
    }

    #[test]
    fn remove_self_from_bucket() {
        execute_test(|fixture: &UserBucketRelationFixture, db: BoxedRepository| {
           let new_relation = NewBucketUserRelation {
                user_uuid: fixture.user2.uuid,
                bucket_uuid: fixture.bucket.uuid,
                set_public_permission: false,
                set_drawing_permission: false,
                set_exclusive_permission: false,
                kick_permission: false,
                grant_permissions_permission: false,
            };
            db.add_user_to_bucket(new_relation).expect("Should add user to bucket");

            let query = UserUuidQueryParam {
                user_uuid: fixture.user2.uuid
            };

            remove_user_from_bucket_handler(fixture.bucket.uuid, fixture.user2.uuid, query,  db)
                .expect("User should be removed");
        })
    }

    #[test]
    fn remove_other_from_bucket() {
        execute_test(|fixture: &UserBucketRelationFixture, db: BoxedRepository| {
           let new_relation = NewBucketUserRelation {
                user_uuid: fixture.user2.uuid,
                bucket_uuid: fixture.bucket.uuid,
                set_public_permission: false,
                set_drawing_permission: false,
                set_exclusive_permission: false,
                kick_permission: false,
                grant_permissions_permission: false,
            };
            db.add_user_to_bucket(new_relation).expect("Should add user to bucket");

            let query = UserUuidQueryParam {
                user_uuid: fixture.user2.uuid
            };

            // User 1, who has permissions, should remove user 2.
            remove_user_from_bucket_handler(fixture.bucket.uuid, fixture.user1.uuid, query,  db)
                .expect("User should be removed");
        })
    }

    #[test]
    fn cant_remove_other_from_bucket() {
        execute_test(|fixture: &UserBucketRelationFixture, db: BoxedRepository| {
           let new_relation = NewBucketUserRelation {
                user_uuid: fixture.user2.uuid,
                bucket_uuid: fixture.bucket.uuid,
                set_public_permission: false,
                set_drawing_permission: false,
                set_exclusive_permission: false,
                kick_permission: false,
                grant_permissions_permission: false,
            };
            db.add_user_to_bucket(new_relation).expect("Should add user to bucket");

            let query = UserUuidQueryParam {
                user_uuid: fixture.user1.uuid
            };

            // User 2, who does not have kick permissions, should not be able to remove user 1.
            remove_user_from_bucket_handler(fixture.bucket.uuid, fixture.user2.uuid, query,  db)
                .expect_err("User should not be removed.");
        })
    }
}

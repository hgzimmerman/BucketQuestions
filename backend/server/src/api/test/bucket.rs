//! Tests for bucket apis.
use crate::{
    api::{
        auth::test::get_jwt,
        bucket::{ChangeBucketFlagsRequest, NewBucketRequest, SetPermissionsRequest},
        routes,
    },
    error::ErrorResponse,
    state::{test_util::execute_test_on_repository, State},
    util::test_util::deserialize,
};
use authorization::{AUTHORIZATION_HEADER_KEY, BEARER, Secret};
use db::{
    bucket::db_types::Bucket,
    bucket_user_relation::db_types::{BucketUserPermissions, BucketUserRelation},
    test::{
        bucket_and_user_fixture::BucketAndUserFixture,
        bucket_user_relation_fixture::UserBucketRelationFixture,
    },
    user::db_types::User,
    RepositoryProvider,
};
use warp::{http::status::StatusCode, test::request};

#[test]
fn create_bucket() {
    execute_test_on_repository(
        |_fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);
            let jwt = get_jwt(&state);

            let new_bucket = NewBucketRequest {
                bucket_name: "I'm a bucket".to_string(),
            };

            let expected_slug = "i-m-a-bucket";

            let resp = request()
                .method("POST")
                .json(&new_bucket)
                .header("content-length", "500")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path("/api/bucket")
                .reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let bucket = deserialize::<Bucket>(&resp);
            assert_eq!(bucket.bucket_name, new_bucket.bucket_name);
            assert_eq!(bucket.bucket_slug, expected_slug);
        },
    );
}

#[test]
fn create_bucket_duplicates() {
    execute_test_on_repository(
        |_fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);
            let jwt = get_jwt(&state);

            let new_bucket = NewBucketRequest {
                bucket_name: "I'm a bucket".to_string(),
            };

            let expected_slug = "i-m-a-bucket";
            let resp = request()
                .method("POST")
                .json(&new_bucket)
                .header("content-length", "500")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path("/api/bucket")
                .reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let bucket = deserialize::<Bucket>(&resp);
            assert_eq!(bucket.bucket_name, new_bucket.bucket_name);
            assert_eq!(bucket.bucket_slug, expected_slug);

            let expected_slug = "i-m-a-bucket-0";
            let resp = request()
                .method("POST")
                .json(&new_bucket)
                .header("content-length", "500")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path("/api/bucket")
                .reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let bucket = deserialize::<Bucket>(&resp);
            assert_eq!(bucket.bucket_name, new_bucket.bucket_name);
            assert_eq!(bucket.bucket_slug, expected_slug);

            let expected_slug = "i-m-a-bucket-1";
            let resp = request()
                .method("POST")
                .json(&new_bucket)
                .header("content-length", "500")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path("/api/bucket")
                .reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let bucket = deserialize::<Bucket>(&resp);
            assert_eq!(bucket.bucket_name, new_bucket.bucket_name);
            assert_eq!(bucket.bucket_slug, expected_slug);
        },
    );
}

#[test]
fn get_bucket() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);

            let url = format!("/api/bucket/slug/{}", fix.bucket.bucket_slug);

            let resp = request().method("GET").path(&url).reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let bucket = deserialize::<Bucket>(&resp);
            assert_eq!(fix.bucket.bucket_name, bucket.bucket_name);
            assert_eq!(fix.bucket.bucket_slug, bucket.bucket_slug);
        },
    )
}

#[test]
fn get_bucket_by_uuid() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);

            let url = format!("/api/bucket/{}", fix.bucket.uuid);

            let resp = request().method("GET").path(&url).reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let bucket = deserialize::<Bucket>(&resp);
            assert_eq!(bucket, fix.bucket);
        },
    )
}

#[test]
fn get_buckets_user_is_in() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);

            let jwt = get_jwt(&state);

            let url = "/api/bucket/in";

            let resp = request()
                .method("GET")
                .path(&url)
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let buckets = deserialize::<Vec<Bucket>>(&resp);
            assert_eq!(buckets.len(), 1);
            assert_eq!(buckets[0], fix.bucket);
        },
    )
}

#[test]
fn get_public_buckets() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);

            let url = "/api/bucket/public";

            let resp = request().method("GET").path(&url).reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let buckets = deserialize::<Vec<Bucket>>(&resp);
            assert_eq!(buckets.len(), 1);
            assert_eq!(buckets[0], fix.bucket);
        },
    )
}

#[test]
fn add_self_to_bucket() {
    execute_test_on_repository(|fix: &BucketAndUserFixture, provider: RepositoryProvider| {
        let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
        let filter = routes(&state);
        let jwt = get_jwt(&state);

        let url = format!("/api/bucket/{}/user", fix.bucket.uuid);

        let resp = request()
            .method("POST")
            .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
            .path(&url)
            .reply(&filter);
        println!("{:?}", resp.body());

        assert_eq!(resp.status(), StatusCode::OK);

        let _ = deserialize::<BucketUserRelation>(&resp);
    });
}

#[test]
fn add_self_to_bucket_no_duplicates() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);
            let jwt = get_jwt(&state);

            let url = format!("/api/bucket/{}/user", fix.bucket.uuid);

            let resp = request()
                .method("POST")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path(&url)
                .reply(&filter);
            println!("{:?}", resp.body());

            assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);

            let _error_response = deserialize::<ErrorResponse>(&resp);
        },
    );
}

#[test]
fn get_permissions_for_self() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);
            let jwt = get_jwt(&state);

            let url = format!("/api/bucket/{}/user", fix.bucket.uuid);

            let resp = request()
                .method("GET")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path(&url)
                .reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let res = deserialize::<BucketUserPermissions>(&resp);
            assert_eq!(
                res.set_exclusive_permission,
                fix.relation.set_exclusive_permission
            );
            assert_eq!(
                res.set_public_permission,
                fix.relation.set_public_permission
            );
            assert_eq!(
                res.set_drawing_permission,
                fix.relation.set_drawing_permission
            );
        },
    );
}

#[test]
fn set_permissions() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);
            let jwt = get_jwt(&state);

            let url = format!("/api/bucket/{}/user", fix.bucket.uuid);

            let req = SetPermissionsRequest {
                target_user_uuid: fix.user1.uuid,
                set_public_permission: Some(false),
                set_drawing_permission: None,
                set_exclusive_permission: Some(true),
                grant_permissions_permission: None,
            };

            let resp = request()
                .method("PUT")
                .json(&req)
                .header("content-length", "500")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path(&url)
                .reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let res = deserialize::<BucketUserPermissions>(&resp);
            assert_eq!(res.set_exclusive_permission, true);
            assert_eq!(res.set_public_permission, false);
            assert_eq!(
                res.set_drawing_permission,
                fix.relation.set_drawing_permission
            );
        },
    );
}

#[test]
fn set_bucket_flags() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);
            let jwt = get_jwt(&state);

            let url = format!("/api/bucket/{}", fix.bucket.uuid);

            let req = ChangeBucketFlagsRequest {
                publicly_visible: Some(false),
                drawing_enabled: None,
                exclusive: Some(true),
            };

            let resp = request()
                .method("PUT")
                .json(&req)
                .header("content-length", "500")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path(&url)
                .reply(&filter);
            println!("{:?}", resp);
            assert_eq!(resp.status(), StatusCode::OK);

            let res = deserialize::<Bucket>(&resp);
            assert_eq!(res.exclusive, true);
            assert_eq!(res.public_viewable, false);
            assert_eq!(res.drawing_enabled, fix.bucket.drawing_enabled);
        },
    );
}

#[test]
fn get_users_in_bucket() {
    execute_test_on_repository(
        |fix: &UserBucketRelationFixture, provider: RepositoryProvider| {
            let state = State::testing_init(provider, Secret::new_hmac("hello".to_string()));
            let filter = routes(&state);
            let jwt = get_jwt(&state);

            let url = format!("/api/bucket/{}/users", fix.bucket.uuid);

            let resp = request()
                .method("GET")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path(&url)
                .reply(&filter);

            assert_eq!(resp.status(), StatusCode::OK);

            let res = deserialize::<Vec<User>>(&resp);
            assert_eq!(res.len(), 1);
            assert_eq!(res[0], fix.user1)
        },
    );
}

use crate::{
    bucket_user_relation::db_types::{BucketUserPermissionsChangeset, NewBucketUserRelation},
    test::{bucket_user_relation_fixture::UserBucketRelationFixture, setup},
};

#[test]
fn create_relation() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let new_relation = NewBucketUserRelation {
        user_uuid: fixture.user2.uuid,
        bucket_uuid: fixture.bucket.uuid,
        set_public_permission: false,
        set_drawing_permission: false,
        set_exclusive_permission: false,
        grant_permissions_permission: false,
    };
    db.add_user_to_bucket(new_relation)
        .expect("Should be able to add user to bucket");
}

#[test]
fn cant_create_duplicate_relation() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let new_relation = NewBucketUserRelation {
        user_uuid: fixture.user1.uuid, // User 1 already has a join.
        bucket_uuid: fixture.bucket.uuid,
        set_public_permission: false,
        set_drawing_permission: false,
        set_exclusive_permission: false,
        grant_permissions_permission: false,
    };
    db.add_user_to_bucket(new_relation)
        .expect_err("Should not able to add user to bucket twice");
}

#[test]
fn remove_user_from_bucket() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let relation = db
        .remove_user_from_bucket(fixture.user1.uuid, fixture.bucket.uuid)
        .expect("Should be able to remove user");
    assert_eq!(relation, fixture.relation);
    db.get_user_bucket_relation(fixture.user1.uuid, fixture.bucket.uuid)
        .expect_err("Relation should be deleted");
}

#[test]
fn cant_remove_unrelated_user_from_bucket() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let _relation = db
        .remove_user_from_bucket(fixture.user2.uuid, fixture.bucket.uuid)
        .expect_err("Should not able to remove user not in bucket");
}

#[test]
fn set_permissions() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let changeset = BucketUserPermissionsChangeset {
        user_uuid: fixture.user1.uuid,
        bucket_uuid: fixture.bucket.uuid,
        set_public_permission: None,
        set_drawing_permission: None,
        set_exclusive_permission: None,
        grant_permissions_permission: Some(false),
    };

    assert_eq!(fixture.relation.grant_permissions_permission, true); // precondition

    let relation = db
        .set_permissions(changeset)
        .expect("Should be able to set permissions");
    assert_eq!(relation.grant_permissions_permission, false);
}

#[test]
fn set_empty_permissions() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let changeset = BucketUserPermissionsChangeset {
        user_uuid: fixture.user1.uuid,
        bucket_uuid: fixture.bucket.uuid,
        set_public_permission: None,
        set_drawing_permission: None,
        set_exclusive_permission: None,
        grant_permissions_permission: None,
    };
    let _relation = db
        .set_permissions(changeset)
        .expect("Should be able to set empty permissions");
}

#[test]
fn get_relation() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let relation = db
        .get_user_bucket_relation(fixture.user1.uuid, fixture.bucket.uuid)
        .expect("Should get relation");
    assert_eq!(relation, fixture.relation);
}

#[test]
fn cant_get_relation() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let _relation = db
        .get_user_bucket_relation(fixture.user2.uuid, fixture.bucket.uuid)
        .expect_err("Should not get relation");
}

#[test]
fn get_associated_users() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let users = db
        .get_users_in_bucket(fixture.bucket.uuid)
        .expect("Should get users");
    assert_eq!(users.len(), 1);
    assert_eq!(users.get(0).expect("Should get user"), &fixture.user1);
}

#[test]
fn get_associated_buckets() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let users = db
        .get_buckets_user_is_a_part_of(fixture.user1.uuid)
        .expect("Should get related buckets");
    assert_eq!(users.len(), 1);
    assert_eq!(users.get(0).unwrap(), &fixture.bucket);
}

#[test]
fn dont_get_unassociated_buckets() {
    let (fixture, db) = setup::<UserBucketRelationFixture>();
    let users = db
        .get_buckets_user_is_a_part_of(fixture.user2.uuid)
        .expect("Should get related buckets");
    assert_eq!(users.len(), 0);
}

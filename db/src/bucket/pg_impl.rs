//! Implementation of the specified interfaces for PgConnection.

use crate::bucket::interface::{BucketRepository, BucketUserRelationRepository};
use diesel::pg::PgConnection;
use crate::bucket::db_types::{NewBucket, Bucket, NewBucketUserJoin, BucketUserJoin, BucketUserPermissionsChangeset, BucketUserPermissions};
use diesel::result::Error;
use uuid::Uuid;
use crate::schema::{
    buckets,
    bucket_user_join
};
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;
use diesel::SaveChangesDsl;
use diesel::BelongingToDsl;
use diesel::query_dsl::InternalJoinDsl;
use crate::user::UserRepository;


impl BucketRepository for PgConnection {
    fn create_bucket(&self, new_bucket: NewBucket) -> Result<Bucket, Error> {
        crate::util::create_row(buckets::table, new_bucket, self)
    }

    fn delete_bucket(&self, bucket_uuid: Uuid) -> Result<Bucket, Error> {
        crate::util::delete_row(buckets::table, bucket_uuid, self)
    }

    fn get_publicly_visible_buckets(&self) -> Result<Vec<Bucket>, Error> {
        buckets::table
            .filter(buckets::visible.eq(true))
            .get_results(self)
    }

    fn get_bucket_by_slug(&self, slug: String) -> Result<Bucket, Error> {
        buckets::table
            .filter(&buckets::bucket_slug.eq(slug))
            .first(self)
    }

    fn get_bucket_by_uuid(&self, uuid: Uuid) -> Result<Bucket, Error> {
        crate::util::get_row(buckets::table, uuid, self)
    }

    fn change_visibility(&self, bucket_uuid: Uuid, visible: bool) -> Result<Bucket, Error> {
        let target = buckets::table
            .find(bucket_uuid);

        diesel::update(target)
            .set(buckets::visible.eq(visible))
            .get_result(self)
    }

    fn change_drawing_status(&self, bucket_uuid: Uuid, drawing: bool) -> Result<Bucket, Error> {
        let target = buckets::table
            .find(bucket_uuid);

        diesel::update(target)
            .set(buckets::drawing_enabled.eq(drawing))
            .get_result(self)
    }
}

impl BucketUserRelationRepository for PgConnection {
    fn add_user_to_bucket(&self, relation: NewBucketUserJoin) -> Result<BucketUserJoin, Error> {
        crate::util::create_row(bucket_user_join::table, relation, self)
    }

    fn remove_user_from_bucket(&self, user_uuid: Uuid, bucket_uuid: Uuid) -> Result<BucketUserJoin, Error> {
        crate::util::delete_row(bucket_user_join::table, user_uuid, self)
    }

    fn set_permissions(&self, permissions_changeset: BucketUserPermissionsChangeset) -> Result<BucketUserJoin, Error> {
        permissions_changeset.save_changes(self)
    }

    fn get_permissions(&self, user_uuid: Uuid, bucket_uuid: Uuid) -> Result<BucketUserPermissions, Error> {
        bucket_user_join::table
            .find(user_uuid)
            .select((
                bucket_user_join::uuid,
                bucket_user_join::set_visibility_permission,
                bucket_user_join::set_drawing_permission,
                bucket_user_join::grant_permissions_permission
            ))
            .get_result::<BucketUserPermissions>(self)
    }

    fn get_buckets_user_is_a_part_of(&self, user_uuid: Uuid) -> Result<Vec<Bucket>, Error> {
        bucket_user_join::table
            .filter(bucket_user_join::user_uuid.eq(user_uuid))
            .select(bucket_user_join::bucket_uuid)
            .inner_join(buckets::table)
            .select((buckets::all_columns))
            .get_results(self)
    }
}
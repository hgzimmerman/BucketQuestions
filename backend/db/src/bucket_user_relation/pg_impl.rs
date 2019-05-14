//! Implementation of the specified interfaces for PgConnection.

use crate::bucket_user_relation::interface::BucketUserRelationRepository;
use crate::AsConnRef;
use crate::bucket_user_relation::db_types::{NewBucketUserRelation, BucketUserRelation, BucketUserPermissionsChangeset, BucketUserPermissions};
use diesel::result::Error;
use uuid::Uuid;
use crate::bucket::db_types::Bucket;
use crate::user::db_types::User;
use crate::schema::{bq_user, bucket_user_relation, bucket};
use log::info;
use diesel::query_dsl::{QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;
use diesel::SaveChangesDsl;

impl<T> BucketUserRelationRepository for T
where
    T: AsConnRef,
{
    fn add_user_to_bucket(
        &self,
        relation: NewBucketUserRelation,
    ) -> Result<BucketUserRelation, Error> {
        crate::util::create_row(bucket_user_relation::table, relation, self.as_conn())
    }

    fn remove_user_from_bucket(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> Result<BucketUserRelation, Error> {
        let target = bucket_user_relation::table.filter(
            bucket_user_relation::user_uuid
                .eq(user_uuid)
                .and(bucket_user_relation::bucket_uuid.eq(bucket_uuid)),
        );
        diesel::delete(target).get_result(self.as_conn())
    }

    fn get_user_bucket_relation(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> Result<BucketUserRelation, Error> {
        bucket_user_relation::table
            .filter(
                bucket_user_relation::user_uuid
                    .eq(user_uuid)
                    .and(bucket_user_relation::bucket_uuid.eq(bucket_uuid)),
            )
            .get_result(self.as_conn())
    }

    fn set_permissions(
        &self,
        permissions_changeset: BucketUserPermissionsChangeset,
    ) -> Result<BucketUserRelation, Error> {
        permissions_changeset
            .save_changes(self.as_conn())
            .or_else(|error: Error| {
                // The query will return an error if there are no changes,
                // if that is the case, just fetch the whole bucket.
                match error {
                    Error::QueryBuilderError(_) => self.get_user_bucket_relation(
                        permissions_changeset.user_uuid,
                        permissions_changeset.bucket_uuid,
                    ),
                    other => Err(other),
                }
            })
    }

    fn get_permissions(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> Result<BucketUserPermissions, Error> {
        bucket_user_relation::table
            .filter(
                bucket_user_relation::user_uuid
                    .eq(user_uuid)
                    .and(bucket_user_relation::bucket_uuid.eq(bucket_uuid)),
            )
            .select((
                bucket_user_relation::set_public_permission,
                bucket_user_relation::set_drawing_permission,
                bucket_user_relation::set_exclusive_permission,
                bucket_user_relation::grant_permissions_permission,
            ))
            .get_result::<BucketUserPermissions>(self.as_conn())
    }

    fn get_buckets_user_is_a_part_of(&self, user_uuid: Uuid) -> Result<Vec<Bucket>, Error> {
        info!("get_buckets_user_is_a_part_of");
        bucket_user_relation::table
            .filter(bucket_user_relation::user_uuid.eq(user_uuid))
            .select(bucket_user_relation::bucket_uuid)
            .inner_join(bucket::table)
            .order_by(bucket::updated_at)
            .select(bucket::all_columns)
            .get_results(self.as_conn())
    }

    fn get_users_in_bucket(&self, bucket_uuid: Uuid) -> Result<Vec<User>, Error> {
        info!("get_users_in_bucket");
        bucket_user_relation::table
            .filter(bucket_user_relation::bucket_uuid.eq(bucket_uuid))
            .select(bucket_user_relation::user_uuid)
            .inner_join(bq_user::table)
            .order_by(bucket_user_relation::created_at)
            .select(bq_user::all_columns)
            .get_results(self.as_conn())
    }
}
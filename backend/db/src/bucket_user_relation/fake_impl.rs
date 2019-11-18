//! Mock impl
use crate::{
    bucket::db_types::Bucket,
    bucket_user_relation::{
        db_types::{
            BucketUserPermissions, BucketUserPermissionsChangeset, BucketUserRelation,
            NewBucketUserRelation,
        },
        interface::BucketUserRelationRepository,
    },
    fake::{DummyDbErrorInfo, FakeDatabase},
    user::db_types::User,
};
use diesel::result::{DatabaseErrorKind, Error};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

impl BucketUserRelationRepository for Arc<Mutex<FakeDatabase>> {
    fn add_user_to_bucket(
        &self,
        relation: NewBucketUserRelation,
    ) -> Result<BucketUserRelation, Error> {
        let mut db = self.lock().unwrap();
        let relation = BucketUserRelation {
            user_uuid: relation.user_uuid,
            bucket_uuid: relation.bucket_uuid,
            set_public_permission: relation.set_public_permission,
            set_drawing_permission: relation.set_drawing_permission,
            set_exclusive_permission: relation.set_exclusive_permission,
            kick_permission: relation.kick_permission,
            grant_permissions_permission: relation.grant_permissions_permission,
            updated_at: chrono::Utc::now().naive_utc(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        if db
            .user_bucket_relations
            .iter()
            .find(|r| r.user_uuid == relation.user_uuid && r.bucket_uuid == relation.bucket_uuid)
            .is_some()
        {
            return Err(Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                Box::new(DummyDbErrorInfo::new()),
            ));
        }
        db.user_bucket_relations.push(relation.clone());
        return Ok(relation);
    }

    fn remove_user_from_bucket(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> Result<BucketUserRelation, Error> {
        let mut db = self.lock().unwrap();
        let index = db
            .user_bucket_relations
            .iter()
            .position(|r| r.user_uuid == user_uuid && r.bucket_uuid == bucket_uuid)
            .ok_or_else(|| Error::NotFound)?;

        Ok(db.user_bucket_relations.remove(index))
    }

    fn get_user_bucket_relation(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> Result<BucketUserRelation, Error> {
        let db = self.lock().unwrap();
        db.user_bucket_relations
            .iter()
            .find(|r| r.user_uuid == user_uuid && r.bucket_uuid == bucket_uuid)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }

    fn set_permissions(
        &self,
        permissions_changeset: BucketUserPermissionsChangeset,
    ) -> Result<BucketUserRelation, Error> {
        let mut db = self.lock().unwrap();
        let mut relation = db
            .user_bucket_relations
            .iter_mut()
            .find(|r| {
                r.user_uuid == permissions_changeset.user_uuid
                    && r.bucket_uuid == permissions_changeset.bucket_uuid
            })
            .ok_or_else(|| Error::NotFound)?;

        if let Some(visible) = permissions_changeset.set_public_permission {
            relation.set_public_permission = visible;
        }
        if let Some(drawing_enabled) = permissions_changeset.set_drawing_permission {
            relation.set_drawing_permission = drawing_enabled;
        }
        if let Some(exclusive) = permissions_changeset.set_exclusive_permission {
            relation.set_exclusive_permission = exclusive;
        }
        if let Some(admin) = permissions_changeset.grant_permissions_permission {
            relation.grant_permissions_permission = admin
        }

        Ok(relation.clone())
    }

    fn get_permissions(
        &self,
        user_uuid: Uuid,
        bucket_uuid: Uuid,
    ) -> Result<BucketUserPermissions, Error> {
        self.get_user_bucket_relation(user_uuid, bucket_uuid)
            .map(|r| BucketUserPermissions {
                set_public_permission: r.set_public_permission,
                set_drawing_permission: r.set_drawing_permission,
                set_exclusive_permission: r.set_exclusive_permission,
                grant_permissions_permission: r.grant_permissions_permission,
                kick_permission: r.kick_permission,
            })
    }

    fn get_buckets_user_is_a_part_of(&self, user_uuid: Uuid) -> Result<Vec<Bucket>, Error> {
        let db = self.lock().unwrap();
        let bucket_uuids: Vec<Uuid> = db
            .user_bucket_relations
            .iter()
            .filter(|r| r.user_uuid == user_uuid)
            .map(|r| r.bucket_uuid)
            .collect();
        let buckets = db
            .buckets
            .iter()
            .filter(|b| bucket_uuids.iter().any(|uuid| &b.uuid == uuid))
            .cloned()
            .collect();
        Ok(buckets)
    }

    fn get_users_in_bucket(&self, bucket_uuid: Uuid) -> Result<Vec<User>, Error> {
        let db = self.lock().unwrap();
        let user_uuids: Vec<Uuid> = db
            .user_bucket_relations
            .iter()
            .filter(|r| r.bucket_uuid == bucket_uuid)
            .map(|r| r.user_uuid)
            .collect();
        let users = db
            .users
            .iter()
            .filter(|b| user_uuids.iter().any(|uuid| &b.uuid == uuid))
            .cloned()
            .collect();

        Ok(users)
    }

    fn get_permissions_all_users_in_bucket(&self, bucket_uuid: Uuid) -> Result<Vec<(BucketUserPermissions, User)>, Error> {
        let db = self.lock().unwrap();
        let users = self.get_users_in_bucket(bucket_uuid)?;

        // unlock
        drop(db);

        let permissions: Vec<BucketUserPermissions> = users
            .iter()
            .map(|user| {
                self.get_permissions(user.uuid, bucket_uuid)
            })
            .collect::<Result<Vec<BucketUserPermissions>, Error>>()?;

        Ok(permissions.into_iter().zip(users).collect())


    }
}

//! Bucket-User relation conversions

use crate::bucket_user_relation::db_types::{
    BucketUserPermissionsChangeset, BucketUserRelation, NewBucketUserRelation,
};
use wire;

impl Into<wire::bucket_user_relation::BucketUserRelation> for BucketUserRelation {
    fn into(self) -> wire::bucket_user_relation::BucketUserRelation {
        wire::bucket_user_relation::BucketUserRelation {
            user_uuid: self.user_uuid,
            bucket_uuid: self.bucket_uuid,
            set_public_permission: self.set_public_permission,
            set_drawing_permission: self.set_drawing_permission,
            set_exclusive_permission: self.set_exclusive_permission,
            kick_permission: self.kick_permission,
            grant_permissions_permission: self.grant_permissions_permission,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

impl From<wire::bucket_user_relation::BucketUserRelation> for BucketUserRelation {
    fn from(bur: wire::bucket_user_relation::BucketUserRelation) -> Self {
        BucketUserRelation {
            user_uuid: bur.user_uuid,
            bucket_uuid: bur.bucket_uuid,
            set_public_permission: bur.set_public_permission,
            set_drawing_permission: bur.set_drawing_permission,
            set_exclusive_permission: bur.set_exclusive_permission,
            kick_permission: bur.kick_permission,
            grant_permissions_permission: bur.grant_permissions_permission,
            updated_at: bur.updated_at,
            created_at: bur.created_at,
        }
    }
}

impl Into<wire::bucket_user_relation::NewBucketUserRelation> for NewBucketUserRelation {
    fn into(self) -> wire::bucket_user_relation::NewBucketUserRelation {
        wire::bucket_user_relation::NewBucketUserRelation {
            user_uuid: self.user_uuid,
            bucket_uuid: self.bucket_uuid,
            set_public_permission: self.set_public_permission,
            set_drawing_permission: self.set_drawing_permission,
            set_exclusive_permission: self.set_exclusive_permission,
            kick_permission: self.kick_permission,
            grant_permissions_permission: self.grant_permissions_permission,
        }
    }
}

impl From<wire::bucket_user_relation::NewBucketUserRelation> for NewBucketUserRelation {
    fn from(bur: wire::bucket_user_relation::NewBucketUserRelation) -> Self {
        NewBucketUserRelation {
            user_uuid: bur.user_uuid,
            bucket_uuid: bur.bucket_uuid,
            set_public_permission: bur.set_public_permission,
            set_drawing_permission: bur.set_drawing_permission,
            set_exclusive_permission: bur.set_exclusive_permission,
            kick_permission: bur.kick_permission,
            grant_permissions_permission: bur.grant_permissions_permission,
        }
    }
}

impl Into<wire::bucket_user_relation::BucketUserPermissionsChangeset>
    for BucketUserPermissionsChangeset
{
    fn into(self) -> wire::bucket_user_relation::BucketUserPermissionsChangeset {
        wire::bucket_user_relation::BucketUserPermissionsChangeset {
            user_uuid: self.user_uuid,
            bucket_uuid: self.bucket_uuid,
            set_public_permission: self.set_public_permission,
            set_drawing_permission: self.set_drawing_permission,
            set_exclusive_permission: self.set_exclusive_permission,
            kick_permission: self.kick_permission,
            grant_permissions_permission: self.grant_permissions_permission,
        }
    }
}

impl From<wire::bucket_user_relation::BucketUserPermissionsChangeset>
    for BucketUserPermissionsChangeset
{
    fn from(bupc: wire::bucket_user_relation::BucketUserPermissionsChangeset) -> Self {
        BucketUserPermissionsChangeset {
            user_uuid: bupc.user_uuid,
            bucket_uuid: bupc.bucket_uuid,
            set_public_permission: bupc.set_public_permission,
            set_drawing_permission: bupc.set_drawing_permission,
            set_exclusive_permission: bupc.set_exclusive_permission,
            kick_permission: bupc.kick_permission,
            grant_permissions_permission: bupc.grant_permissions_permission,
        }
    }
}

use super::*;
use uuid::Uuid;
use wire::bucket_user_relation::{BucketUserRelation, BucketUserPermissions, UserAndPermissions};
use wire::bucket::{SetPermissionsRequest, ChangeBucketFlagsRequest};
use wire::user::User;

/// Creates a bucket
#[derive(Clone, Debug)]
pub struct CreateBucket(pub NewBucketRequest);

impl FetchRequest for CreateBucket {
    type RequestType = NewBucketRequest;
    type ResponseType = Bucket;

    fn url(&self) -> String {
        create_url("bucket")
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Post(&self.0)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

/// Gets buckets in the public.
pub struct GetPublicBuckets;

impl FetchRequest for GetPublicBuckets {
    type RequestType = ();
    type ResponseType = Vec<Bucket>;

    fn url(&self) -> String {
        create_url("bucket/public")
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

/// Gets buckets that that the user has participated in
///
/// `buckets/in`
pub struct GetParticipatingBuckets;

impl FetchRequest for GetParticipatingBuckets {
    type RequestType = ();
    type ResponseType = Vec<Bucket>;

    fn url(&self) -> String {
        create_url("bucket/in")
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct GetBucketBySlug{pub slug: String}

impl FetchRequest for GetBucketBySlug {
    type RequestType = ();
    type ResponseType = Bucket;

    fn url(&self) -> String {
        create_url(&format!("bucket/slug/{}", self.slug))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct GetBucketByUuid{uuid: Uuid}

impl FetchRequest for GetBucketByUuid {
    type RequestType = ();
    type ResponseType = Bucket;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}", self.uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct AddSelfToBucket{bucket_uuid: Uuid}

impl FetchRequest for AddSelfToBucket {
    type RequestType = ();
    type ResponseType = BucketUserRelation;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/user", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Post(&())
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct RemoveSelfFromBucket{pub bucket_uuid: Uuid}

impl FetchRequest for RemoveSelfFromBucket {
    type RequestType = ();
    type ResponseType = BucketUserRelation;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/user", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Delete
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct GetPermissionsForUser{pub bucket_uuid: Uuid}

impl FetchRequest for GetPermissionsForUser {
    type RequestType = ();
    type ResponseType = BucketUserPermissions;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/user", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct SetPermissionsForUser{pub bucket_uuid: Uuid, permissions: SetPermissionsRequest}

impl FetchRequest for SetPermissionsForUser {
    type RequestType = SetPermissionsRequest;
    type ResponseType = BucketUserRelation;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/user", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Put(&self.permissions)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct SetBucketFlags{pub bucket_uuid: Uuid, pub flag_changeset: ChangeBucketFlagsRequest}

impl FetchRequest for SetBucketFlags {
    type RequestType = ChangeBucketFlagsRequest;
    type ResponseType = Bucket;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Put(&self.flag_changeset)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct GetUsersInBucket{pub bucket_uuid: Uuid}

impl FetchRequest for GetUsersInBucket {
    type RequestType = ();
    type ResponseType = Vec<User>;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/users", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}


pub struct GetUsersAndPermissionsInBucket{pub bucket_uuid: Uuid}

impl FetchRequest for GetUsersAndPermissionsInBucket {
    type RequestType = ();
    type ResponseType = Vec<UserAndPermissions>;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/all_user_permissions", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

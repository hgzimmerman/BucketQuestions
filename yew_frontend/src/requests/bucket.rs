use super::*;
use uuid::Uuid;
use wire::bucket_user_relation::{BucketUserRelation, BucketUserPermissions, UserAndPermissions};
use wire::bucket::{SetPermissionsRequest, ChangeBucketFlagsRequest};
use wire::user::User;

/// Creates a bucket
#[derive(Clone, Debug)]
pub struct CreateBucket(pub NewBucketRequest);

impl FetchRequest for CreateBucket {
    type RequestBody = NewBucketRequest;
    type ResponseBody = Bucket;

    fn url(&self) -> String {
        create_url("bucket")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Post(&self.0)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

/// Gets buckets in the public.
pub struct GetPublicBuckets;

impl FetchRequest for GetPublicBuckets {
    type RequestBody = ();
    type ResponseBody = Vec<Bucket>;

    fn url(&self) -> String {
        create_url("bucket/public")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

/// Gets buckets that that the user has participated in
///
/// `buckets/in`
pub struct GetParticipatingBuckets;

impl FetchRequest for GetParticipatingBuckets {
    type RequestBody = ();
    type ResponseBody = Vec<Bucket>;

    fn url(&self) -> String {
        create_url("bucket/in")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetBucketBySlug{pub slug: String}

impl FetchRequest for GetBucketBySlug {
    type RequestBody = ();
    type ResponseBody = Bucket;

    fn url(&self) -> String {
        create_url(&format!("bucket/slug/{}", self.slug))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetBucketByUuid{pub uuid: Uuid}

impl FetchRequest for GetBucketByUuid {
    type RequestBody = ();
    type ResponseBody = Bucket;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}", self.uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct AddSelfToBucket{pub bucket_uuid: Uuid}

impl FetchRequest for AddSelfToBucket {
    type RequestBody = ();
    type ResponseBody = BucketUserRelation;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/user", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Post(&())
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct RemoveSelfFromBucket{pub bucket_uuid: Uuid}

impl FetchRequest for RemoveSelfFromBucket {
    type RequestBody = ();
    type ResponseBody = BucketUserRelation;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/user", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Delete
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetPermissionsForUser{pub bucket_uuid: Uuid}

impl FetchRequest for GetPermissionsForUser {
    type RequestBody = ();
    type ResponseBody = BucketUserPermissions;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/user", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct SetPermissionsForUser{pub bucket_uuid: Uuid, permissions: SetPermissionsRequest}

impl FetchRequest for SetPermissionsForUser {
    type RequestBody = SetPermissionsRequest;
    type ResponseBody = BucketUserRelation;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/user", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Put(&self.permissions)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct SetBucketFlags{pub bucket_uuid: Uuid, pub flag_changeset: ChangeBucketFlagsRequest}

impl FetchRequest for SetBucketFlags {
    type RequestBody = ChangeBucketFlagsRequest;
    type ResponseBody = Bucket;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Put(&self.flag_changeset)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetUsersInBucket{pub bucket_uuid: Uuid}

impl FetchRequest for GetUsersInBucket {
    type RequestBody = ();
    type ResponseBody = Vec<User>;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/users", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}


pub struct GetUsersAndPermissionsInBucket{pub bucket_uuid: Uuid}

impl FetchRequest for GetUsersAndPermissionsInBucket {
    type RequestBody = ();
    type ResponseBody = Vec<UserAndPermissions>;

    fn url(&self) -> String {
        create_url(&format!("bucket/{}/all_user_permissions", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

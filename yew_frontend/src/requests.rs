use crate::common::{FetchRequest, MethodBody};
use serde::{Serialize, Deserialize};
use wire::user::BEARER;

use wire::bucket::{Bucket, NewBucketRequest};




// TODO move this into wire.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkResponse {
    pub link: String,
}

pub fn plain_jwt_header() -> Vec<(String, String)> {
    if let Some(jwt) = crate::auth::get_jwt() {
        vec! [("AUTHORIZATION".to_string(), [BEARER, &jwt].into_iter().cloned().collect())]
    } else {
        log::warn!("Attempting to attach jwt, but it is not in local storage");
        vec![]
    }
}

pub fn cors_access_control_header() -> Vec<(String, String)> {
    vec! [("Access-Control-Allow-Origin".to_string(), "*".to_string())] // TODO restrict this to a more sane default
}

pub fn default_headers() -> Vec<(String, String)> {
    let mut headers = vec![];
    headers.extend(plain_jwt_header());
    headers.extend(cors_access_control_header());

    headers
}

const URL_BASE: &str = "http://0.0.0.0:8080/api/";

fn create_url(path: &str) -> String {
    [URL_BASE, path].into_iter().cloned().collect()
}

pub mod auth_and_user {
    use super::*;
    use uuid::Uuid;

    pub struct GetOauthLink;
    impl FetchRequest for GetOauthLink {
        type RequestType = ();
        type ResponseType = LinkResponse;

        fn url(&self) -> String {
            [URL_BASE, "auth/link"].into_iter().cloned().collect()
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Get
        }

        fn headers(&self) -> Vec<(String, String)> {
            vec![]
        }
    }


    /// Gets user
    pub struct GetUser;
    impl FetchRequest for GetUser {
        type RequestType = ();
        type ResponseType = wire::user::User;

        fn url(&self) -> String {
            create_url("user")
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Get
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct GetUserUuid;
    impl FetchRequest for GetUserUuid {
        type RequestType = ();
        type ResponseType = Uuid;

        fn url(&self) -> String {
            create_url("user/uuid")
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Get
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }
}

pub mod bucket {
    use super::*;
    use uuid::Uuid;
    use wire::bucket_user_relation::BucketUserRelation;
    use wire::bucket::{SetPermissionsRequest, ChangeBucketFlagsRequest};
    use wire::user::User;

    /// Creates a bucket
    pub struct CreateBucket(NewBucketRequest);
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

    pub struct GetBucketBySlug{slug: String}
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

    pub struct RemoveSelfFromBucket{bucket_uuid: Uuid}
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

    pub struct GetPermissionsForUser{bucket_uuid: Uuid}
    impl FetchRequest for GetPermissionsForUser {
        type RequestType = ();
        type ResponseType = BucketUserRelation;

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

    pub struct SetPermissionsForUser{bucket_uuid: Uuid, permissions: SetPermissionsRequest}
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

    pub struct SetBucketFlags{bucket_uuid: Uuid, flag_changeset: ChangeBucketFlagsRequest}
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

    pub struct GetUsersInBucket{bucket_uuid: Uuid}
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
}

pub mod question {
    use super::*;
    use wire::question::{NewQuestionRequest, Question, SetArchivedRequest};
    use uuid::Uuid;

    pub struct CreateQuestion{new_question: NewQuestionRequest}
    impl FetchRequest for CreateQuestion {
        type RequestType = NewQuestionRequest;
        type ResponseType = Question;

        fn url(&self) -> String {
            create_url("question")
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Post(&self.new_question)
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct DeleteQuestion { question_uuid: Uuid }
    impl FetchRequest for DeleteQuestion {
        type RequestType = ();
        type ResponseType = Question;

        fn url(&self) -> String {
            create_url(&format!("question/{}", self.question_uuid))
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Delete
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct GetRandomQuestion;
    impl FetchRequest for GetRandomQuestion {
        type RequestType = ();
        type ResponseType = Option<Question>;

        fn url(&self) -> String {
           create_url("question/random")
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Get
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct GetNumberOfQeustionsInTheBucket{bucket_uuid: Uuid}
    impl FetchRequest for GetNumberOfQeustionsInTheBucket {
        type RequestType = ();
        type ResponseType = usize;

        fn url(&self) -> String {
            create_url(&format!("question/number?bucket_uuid={}", self.bucket_uuid))
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Get
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct GetEveryQuestionInBucket{bucket_uuid: Uuid}
    impl FetchRequest for GetEveryQuestionInBucket {
        type RequestType = ();
        type ResponseType = Vec<Question>;

        fn url(&self) -> String {
            create_url(&format!("question/in_bucket?bucket_uuid={}", self.bucket_uuid))
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Get
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct GetEveryQuestionOnFloor{bucket_uuid: Uuid}
    impl FetchRequest for GetEveryQuestionOnFloor {
        type RequestType = ();
        type ResponseType = Vec<Question>;

        fn url(&self) -> String {
            create_url(&format!("question/on_floor?bucket_uuid={}", self.bucket_uuid))
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Get
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct SetQuestionArchivedState(SetArchivedRequest);
    impl FetchRequest for SetQuestionArchivedState {
        type RequestType = SetArchivedRequest;
        type ResponseType = Question;

        fn url(&self) -> String {
            create_url("question/archive")
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Put(&self.0)
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct SetQuestionAsFavorite{question_uuid: Uuid}
    impl FetchRequest for SetQuestionAsFavorite {
        type RequestType = ();
        type ResponseType = ();

        fn url(&self) -> String {
            create_url(&format!("question/{}/favorite", self.question_uuid))
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Post(&())
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    /// Marks the question as no longer a favorite of the user.
    pub struct RemoveQuestionAsFavorite{question_uuid: Uuid}
    impl FetchRequest for RemoveQuestionAsFavorite {
        type RequestType = ();
        type ResponseType = ();

        fn url(&self) -> String {
            create_url(&format!("question/{}/favorite", self.question_uuid))
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Delete
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }

    pub struct GetFavoriteQuestions;
    impl FetchRequest for GetFavoriteQuestions {
        type RequestType = ();
        type ResponseType = Vec<Question>;

        fn url(&self) -> String {
            create_url("question/favorites")
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Get
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }
}

pub mod answer {
    use super::*;
    use wire::answer::{NewAnswerRequest, Answer};

    pub struct CreateAnswer(NewAnswerRequest);
    impl FetchRequest for CreateAnswer {
        type RequestType = NewAnswerRequest;
        type ResponseType = Answer;

        fn url(&self) -> String {
            create_url("answer")
        }

        fn method(&self) -> MethodBody<Self::RequestType> {
            MethodBody::Post(&self.0)
        }

        fn headers(&self) -> Vec<(String, String)> {
            default_headers()
        }
    }
}
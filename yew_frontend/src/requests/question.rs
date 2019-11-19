use super::*;
use wire::question::{NewQuestionRequest, Question, SetArchivedRequest};
use uuid::Uuid;

pub struct CreateQuestion{pub new_question: NewQuestionRequest}

impl FetchRequest for CreateQuestion {
    type RequestBody = NewQuestionRequest;
    type ResponseBody = Question;

    fn url(&self) -> String {
        create_url("question")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Post(&self.new_question)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct DeleteQuestion {pub question_uuid: Uuid }

impl FetchRequest for DeleteQuestion {
    type RequestBody = ();
    type ResponseBody = Question;

    fn url(&self) -> String {
        create_url(&format!("question/{}", self.question_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Delete
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetRandomQuestion{pub bucket_uuid: Uuid}
impl FetchRequest for GetRandomQuestion {
    type RequestBody = ();
    type ResponseBody = Option<Question>;

    fn url(&self) -> String {
       create_url(&format!("question/random?bucket_uuid={}", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetNumberOfQeustionsInTheBucket{pub bucket_uuid: Uuid}

impl FetchRequest for GetNumberOfQeustionsInTheBucket {
    type RequestBody = ();
    type ResponseBody = usize;

    fn url(&self) -> String {
        create_url(&format!("question/number?bucket_uuid={}", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetEveryQuestionInBucket{pub bucket_uuid: Uuid}

impl FetchRequest for GetEveryQuestionInBucket {
    type RequestBody = ();
    type ResponseBody = Vec<Question>;

    fn url(&self) -> String {
        create_url(&format!("question/in_bucket?bucket_uuid={}", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetEveryQuestionOnFloor{pub bucket_uuid: Uuid}

impl FetchRequest for GetEveryQuestionOnFloor {
    type RequestBody = ();
    type ResponseBody = Vec<Question>;

    fn url(&self) -> String {
        create_url(&format!("question/on_floor?bucket_uuid={}", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct SetQuestionArchivedState(pub SetArchivedRequest);

impl FetchRequest for SetQuestionArchivedState {
    type RequestBody = SetArchivedRequest;
    type ResponseBody = Question;

    fn url(&self) -> String {
        create_url("question/archive")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Put(&self.0)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct SetQuestionAsFavorite{pub question_uuid: Uuid}

impl FetchRequest for SetQuestionAsFavorite {
    type RequestBody = ();
    type ResponseBody = ();

    fn url(&self) -> String {
        create_url(&format!("question/{}/favorite", self.question_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Post(&())
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

/// Marks the question as no longer a favorite of the user.
pub struct RemoveQuestionAsFavorite{pub question_uuid: Uuid}

impl FetchRequest for RemoveQuestionAsFavorite {
    type RequestBody = ();
    type ResponseBody = ();

    fn url(&self) -> String {
        create_url(&format!("question/{}/favorite", self.question_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Delete
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetFavoriteQuestions;

impl FetchRequest for GetFavoriteQuestions {
    type RequestBody = ();
    type ResponseBody = Vec<Question>;

    fn url(&self) -> String {
        create_url("question/favorites")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

use super::*;
use wire::question::{NewQuestionRequest, Question, SetArchivedRequest};
use uuid::Uuid;

pub struct CreateQuestion{pub new_question: NewQuestionRequest}

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

pub struct DeleteQuestion {pub question_uuid: Uuid }

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

pub struct GetRandomQuestion{pub bucket_uuid: Uuid}
impl FetchRequest for GetRandomQuestion {
    type RequestType = ();
    type ResponseType = Option<Question>;

    fn url(&self) -> String {
       create_url(&format!("question/random?bucket_uuid={}", self.bucket_uuid))
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

pub struct GetNumberOfQeustionsInTheBucket{pub bucket_uuid: Uuid}

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

pub struct GetEveryQuestionInBucket{pub bucket_uuid: Uuid}

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

pub struct GetEveryQuestionOnFloor{pub bucket_uuid: Uuid}

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

pub struct SetQuestionArchivedState(pub SetArchivedRequest);

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

pub struct SetQuestionAsFavorite{pub question_uuid: Uuid}

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
pub struct RemoveQuestionAsFavorite{pub question_uuid: Uuid}

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

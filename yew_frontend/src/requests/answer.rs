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

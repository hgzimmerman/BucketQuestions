use super::*;
use wire::answer::{NewAnswerRequest, Answer};

pub struct CreateAnswer(pub NewAnswerRequest);

impl FetchRequest for CreateAnswer {
    type RequestBody = NewAnswerRequest;
    type ResponseBody = Answer;

    fn url(&self) -> String {
        create_url("answer")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Post(&self.0)
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

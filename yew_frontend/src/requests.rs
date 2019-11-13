use crate::common::{FetchRequest, MethodBody};
use serde::{Serialize, Deserialize};


// TODO move this into wire.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkResponse {
    pub link: String,
}

const URL_BASE: &str = "http://0.0.0.0:8080/api/";

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
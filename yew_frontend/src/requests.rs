use crate::common::{FetchRequest, MethodBody};
use serde::{Serialize, Deserialize};
use wire::user::BEARER;




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


pub struct GetUser;

impl FetchRequest for GetUser {
    type RequestType = ();
    type ResponseType = wire::user::User;

    fn url(&self) -> String {
        [URL_BASE, "user"].into_iter().cloned().collect()
    }

    fn method(&self) -> MethodBody<Self::RequestType> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }
}

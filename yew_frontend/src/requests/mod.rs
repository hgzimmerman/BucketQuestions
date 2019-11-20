use yewtil::fetch::{FetchRequest, MethodBody};
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

pub fn json_content_type_header() -> Vec<(String, String)> {
    vec! [("Content-Type".to_string(), "application/json".to_string())]
}

pub fn cors_access_control_header() -> Vec<(String, String)> {
    vec! [("Access-Control-Allow-Origin".to_string(), "*".to_string())] // TODO restrict this to a more sane default
}

pub fn default_headers() -> Vec<(String, String)> {
    let mut headers = vec![];
    headers.extend(plain_jwt_header());
    headers.extend(cors_access_control_header());
    headers.extend(json_content_type_header());

    headers
}

const URL_BASE: &str = "http://0.0.0.0:8080/api/";

fn create_url(path: &str) -> String {
    [URL_BASE, path].into_iter().cloned().collect()
}

// TODO this should be compiled later.
pub fn cors() -> bool {
    true
}

pub mod auth_and_user;

pub mod bucket;

pub mod question;

pub mod answer;
use super::*;
use uuid::Uuid;

pub struct GetOauthLink;

impl FetchRequest for GetOauthLink {
    type RequestBody = ();
    type ResponseBody = LinkResponse;

    fn url(&self) -> String {
        [URL_BASE, "auth/link"].into_iter().cloned().collect()
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![]
    }

    fn use_cors(&self) -> bool {cors()}
}


/// Gets user
pub struct GetUser;

impl FetchRequest for GetUser {
    type RequestBody = ();
    type ResponseBody = wire::user::User;

    fn url(&self) -> String {
        create_url("user")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

pub struct GetUserUuid;

impl FetchRequest for GetUserUuid {
    type RequestBody = ();
    type ResponseBody = Uuid;

    fn url(&self) -> String {
        create_url("user/uuid")
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        default_headers()
    }

    fn use_cors(&self) -> bool {cors()}
}

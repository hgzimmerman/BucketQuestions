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

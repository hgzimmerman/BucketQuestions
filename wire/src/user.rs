use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NewUser {
    /// The user's unique identifier provided by google
    pub google_user_id: String,
    /// The user's name as it appears in google
    pub google_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct User {
    /// The user's unique identifier within the application.
    pub uuid: Uuid,
    /// The user's unique identifier provided by google.
    pub google_user_id: String,
    /// The user's name as it appears in google
    pub google_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JwtPayload<T> {
    /// Issue date of the token
    pub iat: NaiveDateTime,
    /// Subject - the information being authenticated by this token
    pub sub: T,
    /// Expiration date of the token
    pub exp: NaiveDateTime,
}

/// Bearer string.
pub const BEARER: &str = "Bearer ";

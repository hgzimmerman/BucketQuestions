use serde::{Serialize, Deserialize};
use uuid::Uuid;

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
//! Module for user related database interactions.
use uuid::Uuid;

use crate::schema::bq_user;
use serde::{Deserialize, Serialize};

/// A struct representing all the columns in the `users` table.
#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "bq_user"]
pub struct User {
    /// The user's unique identifier within the application.
    pub uuid: Uuid,
    /// The user's unique identifier provided by google.
    pub google_user_id: String,
    /// The user's name as it appears in google
    pub google_name: Option<String>,
}

/// Structure used to create new users.
#[derive(Clone, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "bq_user"]
pub struct NewUser {
    /// The user's unique identifier provided by google
    pub google_user_id: String,
    /// The user's name as it appears in google
    pub google_name: Option<String>,
}

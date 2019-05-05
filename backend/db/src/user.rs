//! All database queries directly related to users are contained within this module.
use crate::schema::{self, users};
use diesel::{
    pg::PgConnection,
    query_dsl::QueryDsl,
    result::{Error, QueryResult},
    ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A struct representing all the columns in the `users` table.
#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "users"]
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
#[table_name = "users"]
pub struct NewUser {
    /// The user's unique identifier provided by google
    pub google_user_id: String,
    /// The user's name as it appears in google
    pub google_name: Option<String>,
}

/// Trait for storing and retrieving users.
pub trait UserRepository {
    /// Creates a user
    fn create_user(&self, user: NewUser) -> QueryResult<User>;
    /// Gets a user using its unique identifier.
    fn get_user(&self, uuid: Uuid) -> QueryResult<User>;
    /// Gets a user by the client id.
    fn get_user_by_google_id(&self, id: String) -> QueryResult<User>;
}

impl UserRepository for PgConnection {
    fn create_user(&self, user: NewUser) -> Result<User, Error> {
        crate::util::create_row(schema::users::table, user, self)
    }
    fn get_user(&self, uuid: Uuid) -> Result<User, Error> {
        crate::util::get_row(schema::users::table, uuid, self)
    }
    fn get_user_by_google_id(&self, id: String) -> Result<User, Error> {
        users::table
            .filter(users::dsl::google_user_id.eq(id))
            .first::<User>(self)
    }
}

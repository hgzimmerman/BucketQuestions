//! Interface for storing users.
use crate::user::db_types::{NewUser, User};
use diesel::QueryResult;
use uuid::Uuid;

/// Trait for storing and retrieving users.
pub trait UserRepository {
    /// Creates a user
    fn create_user(&self, user: NewUser) -> QueryResult<User>;
    /// Gets a user using its unique identifier.
    fn get_user(&self, uuid: Uuid) -> QueryResult<User>;
    /// Gets a user by the client id.
    fn get_user_by_google_id(&self, id: String) -> QueryResult<User>;
}

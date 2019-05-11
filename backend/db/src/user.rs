//! All database queries directly related to users are contained within this module.
use crate::schema::{self, bq_user};
use diesel::{
    query_dsl::QueryDsl,
    result::{Error, QueryResult},
    ExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AsConnRef;

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

/// Trait for storing and retrieving users.
pub trait UserRepository {
    /// Creates a user
    fn create_user(&self, user: NewUser) -> QueryResult<User>;
    /// Gets a user using its unique identifier.
    fn get_user(&self, uuid: Uuid) -> QueryResult<User>;
    /// Gets a user by the client id.
    fn get_user_by_google_id(&self, id: String) -> QueryResult<User>;
}

impl <T> UserRepository for T where T: AsConnRef {
    fn create_user(&self, user: NewUser) -> Result<User, Error> {
        crate::util::create_row(schema::bq_user::table, user, self.as_conn())
    }
    fn get_user(&self, uuid: Uuid) -> Result<User, Error> {
        crate::util::get_row(schema::bq_user::table, uuid, self.as_conn())
    }
    fn get_user_by_google_id(&self, id: String) -> Result<User, Error> {
        bq_user::table
            .filter(bq_user::dsl::google_user_id.eq(id))
            .first::<User>(self.as_conn())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test::{empty_fixture::EmptyFixture, setup, user_fixture::UserFixture},
        user::NewUser,
    };

    #[test]
    fn get_user() {
        let (fixture, db) = setup::<UserFixture>();
        let user = db.get_user(fixture.user.uuid).unwrap();
        assert_eq!(user, fixture.user);
    }

    #[test]
    fn create_get_user() {
        let (_, db) = setup::<EmptyFixture>();
        let new_user = NewUser {
            google_user_id: "12345".to_string(),
            google_name: Some("YEET".to_string()),
        };
        let user = db.create_user(new_user.clone()).unwrap();
        assert_eq!(user.google_user_id, new_user.google_user_id);
        assert_eq!(user.google_name, new_user.google_name);

        let user = db.get_user(user.uuid).unwrap();
        assert_eq!(user.google_user_id, new_user.google_user_id);
        assert_eq!(user.google_name, new_user.google_name);
    }

    #[test]
    fn create_get_by_id_user() {
        let (fixture, db) = setup::<UserFixture>();

        let user = db
            .get_user_by_google_id(fixture.user.google_user_id.clone())
            .unwrap();
        assert_eq!(user.google_name, fixture.user.google_name);
        assert_eq!(user, fixture.user);
    }
}

//! Implementation of the specified interfaces for PgConnection.

use crate::{
    schema::{self, bq_user},
    user::{
        db_types::{NewUser, User},
        interface::UserRepository,
    },
    AsConnRef,
};
use uuid::Uuid;

use diesel::{query_dsl::QueryDsl, result::Error, ExpressionMethods, RunQueryDsl};

impl<T> UserRepository for T
where
    T: AsConnRef,
{
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

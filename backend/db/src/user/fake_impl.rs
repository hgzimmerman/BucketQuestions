//! Mock impl
use crate::{
    fake::{DummyDbErrorInfo, FakeDatabase},
    user::{
        db_types::{NewUser, User},
        interface::UserRepository,
    },
};
use diesel::result::{DatabaseErrorKind, Error};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

impl UserRepository for Arc<Mutex<FakeDatabase>> {
    fn create_user(&self, user: NewUser) -> Result<User, Error> {
        let uuid = Uuid::new_v4();
        let user = User {
            uuid,
            google_user_id: user.google_user_id,
            google_name: user.google_name,
        };
        let mut db = self.lock().unwrap();
        if db.users.iter().find(|u| u.uuid == uuid).is_some() {
            return Err(Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                Box::new(DummyDbErrorInfo::new()),
            ));
        }
        db.users.push(user.clone());
        return Ok(user);
    }

    fn get_user(&self, uuid: Uuid) -> Result<User, Error> {
        let db = self.lock().unwrap();
        db.users
            .iter()
            .find(|u| u.uuid == uuid)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }

    fn get_user_by_google_id(&self, id: String) -> Result<User, Error> {
        let db = self.lock().unwrap();
        db.users
            .iter()
            .find(|u| u.google_user_id == id)
            .cloned()
            .ok_or_else(|| Error::NotFound)
    }
}

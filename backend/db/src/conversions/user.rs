//! User conversions


use wire;
use crate::user::db_types::{User, NewUser};

impl Into<wire::user::User> for User {
    fn into(self) -> wire::user::User {
        wire::user::User {
            uuid: self.uuid,
            google_user_id: self.google_user_id,
            google_name: self.google_name
        }
    }
}

impl From<wire::user::User> for User {
    fn from(user: wire::user::User) -> Self {
        User {
            uuid: user.uuid,
            google_user_id: user.google_user_id,
            google_name: user.google_name
        }
    }
}

impl Into<wire::user::NewUser> for NewUser {
    fn into(self) -> wire::user::NewUser {
        wire::user::NewUser {
            google_user_id: self.google_user_id,
            google_name: self.google_name
        }
    }
}

impl From<wire::user::NewUser> for NewUser {
    fn from(new_user: wire::user::NewUser) -> Self {
        NewUser {
            google_user_id: new_user.google_user_id,
            google_name: new_user.google_name
        }
    }
}
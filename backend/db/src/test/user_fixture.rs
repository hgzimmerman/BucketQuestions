use crate::{
    user::{NewUser, User},
    Repository,
};
use diesel_reset::fixture::Fixture;

/// Fixture that creates one user record in the repository.
pub struct UserFixture {
    pub user: User,
}

impl Fixture for UserFixture {
    type Repository = Box<dyn Repository>;

    fn generate(conn: &Box<Repository>) -> Self {
        let new_user = NewUser {
            google_user_id: "123456789".to_string(),
            google_name: Some("Yeet".to_owned()),
        };

        let user = conn.create_user(new_user).unwrap();

        UserFixture { user }
    }
}

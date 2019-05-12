//! A fixture for testing aganist an existing user.
use crate::{user::{NewUser, User}, Repository, AbstractRepository};
use diesel_reset::fixture::Fixture;

/// Fixture that creates one user record in the repository.
#[derive(Clone, Debug)]
pub struct UserFixture {
    /// User
    pub user: User,
}

impl Fixture for UserFixture {
    type Repository = AbstractRepository;

    fn generate(conn: &AbstractRepository) -> Self {
        let new_user = NewUser {
            google_user_id: "123456789".to_string(),
            google_name: Some("Yeet".to_owned()),
        };

        let user = conn.create_user(new_user).unwrap();

        UserFixture { user }
    }
}

//! A fixture for testing aganist an existing user.
use crate::{user::{NewUser, User}, Repository, AbstractRepository};
use crate::test::fixture::Fixture;

/// Fixture that creates one user record in the repository.
#[derive(Clone, Debug)]
pub struct UserFixture {
    /// User
    pub user: User,
}

/// ID used for testing.
pub const TEST_GOOGLE_USER_ID: &str = "123456789";
/// Name used for testing.
pub const TEST_GOOGLE_NAME: &str = "User";

impl Fixture for UserFixture {
    fn generate(conn: &AbstractRepository) -> Self {
        let new_user = NewUser {
            google_user_id: TEST_GOOGLE_USER_ID.to_string(),
            google_name: Some(TEST_GOOGLE_NAME.to_owned()),
        };

        let user = conn.create_user(new_user).unwrap();

        UserFixture { user }
    }
}

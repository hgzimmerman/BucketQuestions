use crate::user::{User, NewUser, UserRepository};
use diesel::pg::PgConnection;
use crate::test::fixture::Fixture;
use crate::Repository;
use crate::mock::MockDatabase;
use pool::PooledConn;

pub struct UserFixture {
    pub user: User,
}

impl Fixture for UserFixture
{
    type Repository = Box<dyn Repository>;

    fn generate(conn: &Box<Repository>) -> Self  {
        let new_user = NewUser {
            google_user_id: "123456789".to_string(),
            google_name: Some("Yeet".to_owned())
        };

        let user = conn.create_user(new_user).unwrap();

        UserFixture { user }
    }
}

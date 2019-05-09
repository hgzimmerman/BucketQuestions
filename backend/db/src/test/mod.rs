
#[cfg(test)]
mod user_fixture;

#[cfg(test)]
mod tests {
    use crate::Repository;
    use crate::UserRepository;
    use crate::mock::MockDatabase;
    use crate::user::NewUser;
    use std::sync::Mutex;
    use crate::test::user_fixture::{UserFixture, EmptyFixture};
    use pool::PooledConn;
    use std::ops::Deref;
    use diesel::PgConnection;
    use diesel_reset::fixture::Fixture;


    pub fn setup<Fix>() -> (Fix,  Box<Repository>)
    where
        Fix: Fixture<Repository = Box<Repository>>
    {
        // TODO swap this around based on feature flags
        if !cfg!(feature="integration") {
            let db = Mutex::new(MockDatabase::default());
            let db: Box<dyn Repository> = Box::new(db);
            let fixture = Fix::generate(&(db));
            (fixture, db)
        } else {
            let db: PgConnection = diesel_reset::setup::setup_single_connection();
            let db: Box<dyn Repository> = Box::new(db);
            let fixture = Fix::generate(&db);
            (fixture, db)
        }
    }

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
            google_name: Some("YEET".to_string())
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

        let user = db.get_user_by_google_id(fixture.user.google_user_id.clone()).unwrap();
        assert_eq!(user.google_name, fixture.user.google_name);
        assert_eq!(user, fixture.user);
    }

}


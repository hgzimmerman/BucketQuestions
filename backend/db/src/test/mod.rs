
#[cfg(test)]
mod user_fixture;
mod fixture;

#[cfg(test)]
mod tests {
    use crate::Repository;
    use crate::UserRepository;
    use crate::mock::MockDatabase;
    use crate::user::NewUser;
    use std::sync::Mutex;
    use crate::test::user_fixture::UserFixture;
    use pool::PooledConn;
    use std::ops::Deref;
    use diesel::PgConnection;
    use crate::test::fixture::Fixture;

    // TODO consider using a fixture with this.
    #[cfg(test)]
    fn setup() -> impl Repository {
        return Mutex::new(MockDatabase::default())
    }

//    pub fn setup_mock<Fun, Fix>(mut test_function: Fun)
//    where
//        Fun: FnMut(&Fix, Box<Repository>),
//        Fix: Fixture<Repository=Box<Repository>>,
//    {
//
//        test_function(&fixture, db)
//    }

    pub fn s<Fix>() -> (Fix,  Box<Repository>)
    where
        Fix: Fixture<Repository = Box<Repository>>
    {
        // TODO swap this around based on feature flags
        if true {
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

//    macro_rules! setup {
//        () => {{
//        // TODO make this dependent on a feature flag for integration vs unit.
//            if true {
//                setup_mock()
////            } else {
////                diesel_reset::setup::setup($test_function)
//            }
//        }}
//    }



    #[test]
    fn create_get_user() {

//        setup!(|fixture: &UserFixture, db: Box<Repository>| {
//            let new_user = NewUser {
//                google_user_id: "12345".to_string(),
//                google_name: Some("YEET".to_string())
//            };
//            let user = db.create_user(new_user.clone()).unwrap();
//            assert_eq!(user.google_user_id, new_user.google_user_id);
//            assert_eq!(user.google_name, new_user.google_name);
//
//            let user = db.get_user(user.uuid).unwrap();
//            assert_eq!(user.google_user_id, new_user.google_user_id);
//            assert_eq!(user.google_name, new_user.google_name);
//        });

//        let new_user = NewUser {
//            google_user_id: "12345".to_string(),
//            google_name: Some("YEET".to_string())
//        };
//        let user = db.create_user(new_user.clone()).unwrap();
//        assert_eq!(user.google_user_id, new_user.google_user_id);
//        assert_eq!(user.google_name, new_user.google_name);
//
//        let user = db.get_user(user.uuid).unwrap();
//        assert_eq!(user.google_user_id, new_user.google_user_id);
//        assert_eq!(user.google_name, new_user.google_name);
    }


    #[test]
    fn create_get_by_id_user() {
        let db = setup();
        let new_user = NewUser {
            google_user_id: "12345".to_string(),
            google_name: Some("YEET".to_string())
        };
        let user = db.create_user(new_user.clone()).unwrap();
        assert_eq!(user.google_user_id, new_user.google_user_id);
        assert_eq!(user.google_name, new_user.google_name);

        let user = db.get_user_by_google_id(user.google_user_id).unwrap();
        assert_eq!(user.google_name, new_user.google_name);
    }

}


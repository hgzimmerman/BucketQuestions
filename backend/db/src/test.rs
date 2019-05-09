

#[cfg(test)]
mod tests {
    use crate::Repository;
    use crate::UserRepository;
    use crate::mock::MockDatabase;
    use crate::user::NewUser;
    use std::sync::Mutex;

    // TODO consider using a fixture with this.
    #[cfg(test)]
    fn setup() -> impl Repository {
        return Mutex::new(MockDatabase::default())
    }


    #[test]
    fn create_get_user() {
        let db = setup();
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


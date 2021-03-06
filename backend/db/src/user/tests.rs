use crate::{
    test::{empty_fixture::EmptyFixture, user_fixture::UserFixture, util::execute_test},
    user::db_types::NewUser,
    BoxedRepository,
};

#[test]
fn get_user() {
    execute_test(|fixture: &UserFixture, db: BoxedRepository| {
        let user = db.get_user(fixture.user.uuid).unwrap();
        assert_eq!(user, fixture.user);
    })
}

#[test]
fn create_get_user() {
    execute_test(|_fixture: &EmptyFixture, db: BoxedRepository| {
        let new_user = NewUser {
            google_user_id: "12345".to_string(),
            google_name: Some("YEET".to_string()),
        };
        let user = db.create_user(new_user.clone()).unwrap();
        assert_eq!(user.google_user_id, new_user.google_user_id);
        assert_eq!(user.google_name, new_user.google_name);

        let user = db.get_user(user.uuid).unwrap();
        assert_eq!(user.google_user_id, new_user.google_user_id);
        assert_eq!(user.google_name, new_user.google_name);
    });
}

#[test]
fn create_get_by_id_user() {
    execute_test(|fixture: &UserFixture, db: BoxedRepository| {
        let user = db
            .get_user_by_google_id(fixture.user.google_user_id.clone())
            .unwrap();
        assert_eq!(user.google_name, fixture.user.google_name);
        assert_eq!(user, fixture.user);
    });
}

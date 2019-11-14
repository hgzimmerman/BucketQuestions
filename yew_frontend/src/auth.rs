

#[allow(unused)]
mod storage {
    use web_sys::{Storage, window};

    /// Gets a storage object
    fn get_storage() -> Storage {
        window().unwrap().local_storage().unwrap().unwrap()
    }

    /// Sets an item in the storage
    pub fn set_item(key: &str, value: &str) {
        get_storage().set_item(key, value).expect("Should set the kv pair, maybe either is too big.")
    }

    /// Gets an item in the storage
    pub fn get_item(key: &str) -> Option<String> {
        get_storage().get_item(key).expect("Should get value at key, key may have been too big.")
    }

    /// Removes an item in the storage
    pub fn remove_item(key: &str) {
        get_storage().remove_item(key).expect("Should get value at key, key may have been too big.")
    }
}

const JWT_KEY: &str = "jwt";

pub fn clear_jwt() {
    storage::remove_item(JWT_KEY)
}

pub fn get_jwt() -> Option<String> {
    storage::get_item(JWT_KEY)
}

#[allow(unused)]
pub fn is_logged_in() -> bool {
    get_jwt().is_some()
}


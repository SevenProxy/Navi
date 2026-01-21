use std::collections::HashMap;
use web_sys::{
    window,
    Storage,
};

fn get_local_storage() -> Storage {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
}


pub fn set_item(key: &str, value: &str) {
    let storage = get_local_storage();

    storage.set_item(key, value).unwrap();
}


pub fn get_item(key: &str) -> Option<String> {
    let storage = get_local_storage();

    storage.get_item(key).unwrap()
}

pub fn remove_item(key: &str) {
    let storage = get_local_storage();

    storage.remove_item(key).unwrap();
}

pub fn get_all_storage_values() -> HashMap<String, String> {
    let mut all_items = HashMap::new();

    let storage = get_local_storage();
    let length = storage.length().unwrap_or(0);

    for i in 0..length {
        if let Ok(Some(key)) = storage.key(i) {
            if let Ok(Some(value)) = storage.get_item(&key) {
                all_items.insert(key, value);
            }
        }
    }

    all_items
}

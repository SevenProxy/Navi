use yew::prelude::*;
use web_sys::{
    window,
    Storage,
};
use std::collections::HashMap;
use crate::bin::{
    Terminal,
    Lain,
};

fn get_local_storage() -> Storage {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
}

fn set_item(key: &str, value: &str) {
    let storage = get_local_storage();

    storage.set_item(key, value).unwrap();
}

fn get_item(key: &str) -> Option<String> {
    let storage = get_local_storage();

    storage.get_item(key).unwrap()
}

fn remove_item(key: &str) {
    let storage = get_local_storage();

    storage.remove_item(key).unwrap();
}

fn get_all_storage_values() -> HashMap<String, String> {
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

#[component]
pub fn Htop() -> Html {
    let storage_data = use_state(HashMap::new);

    {
        let storage_data = storage_data.clone();
        use_effect_with((), move |_| {
            storage_data.set(get_all_storage_values());
            || ()
        });
    }

    html!{
        <div>
            {
                for storage_data.iter().map(|(k,v)| {
                    match v.as_str() {
                        "terminal" => html!{
                            <Terminal key={k.clone()} />
                        },
                        "lain" => html!{
                            <Lain key={k.clone()}/>
                        },
                        _ => html! { <></> }
                    }
                })
            }
        </div>
    }
}

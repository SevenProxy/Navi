use yew::prelude::*;
use web_sys::{
    window,
    Storage,
    HtmlInputElement,
}


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

[#component]
pub fn Htop() -> Html {

    

    html!{
        
    }
}

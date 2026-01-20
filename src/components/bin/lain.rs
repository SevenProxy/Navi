use yew::prelude::*;
use crate::WindowRoot;
use crate::PropsWindow;

#[component]
pub fn Lain() -> Html {
    let window_props_lain = yew::props! {
        PropsWindow {
            name_window: "Lain".to_string(),
            style_custom: "lain-window z-0".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };


    html!{
        <WindowRoot ..window_props_lain>
            <img class="w-full h-full bg-black" src="https://fauux.neocities.org/16c.gif" />
        </WindowRoot>
    }
}

mod components;

use yew::prelude::*;
use components::{
    header::HeaderRoot,
    navbar::NavbarRoot,
    desktop::DesktopRoot,
    window::{
        WindowRoot,
        PropsWindow,
    },
};


#[component]
fn App() -> Html {

    let window_props_terminal = yew::props! {
        PropsWindow {
            name_window: "Terminal".to_string(),
            style_custom: "w-[600px] h-[500px] z-0".to_string(),
            sub_style: "overflow-y-auto overflow-x-hidden px-5 h-full text-white",
        }
    };

    let window_props_lain = yew::props! {
        PropsWindow {
            name_window: "Lain".to_string(),
            style_custom: "lain-window z-0".to_string(),
            sub_style: "w-full h-full text-white".to_string(),
        }
    };

    html!{
        <div class="bg-black h-screen overflow-hidden">
            <div class="h-full">
                <HeaderRoot />
                <main class="h-full">
                    <DesktopRoot>
                        <WindowRoot ..window_props_terminal>
                            <div>
                                <div class="">
                                    <p>{"Lain @2026"}</p>
                                    <p>{"⠀⠀⠀⠀⣀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡢⡀⠀⠀⠀⠀"}</p>
                                    <p>{"⠀⠀⠀⢄⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦"}</p>
                                    <p>{"⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧"}</p>
                                    <p>{" ⢨⣿⡿⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣇"}</p>
                                    <p>{" ⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢀"}</p>
                                    <p>{"⢈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣾⠀"}</p>
                                    <p>{"⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⡇⠀⢸⢹⣿⣿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡧"}</p>
                                    <p>{"⠈⣿⣿⣿⣿⣿⠧⠯⠟⠿⠧⠀⠀⠀⠸⠿⠿⢼⣿⠿⢿⣟⣿⣿⣿⣿⣿⣇"}</p>
                                    <p>{"⠀⣿⣿⣿⣿⡿⢰⠺⣿⠉⠂⠀⠀⠀⠀⠀⠀⠚⣷⣶⠢⡀⢿⣿⣿⣿⡿⠉"}</p>
                                    <p>{"⢐⢻⣿⣏⠙⠇⠈⠒⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠝⠻⠥⠁⢰⡌⠹⠋⡀⡀"}</p>
                                    <p>{"⠀⠉⢻⣿⣦⡀⠐⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠆⠄⠸⢃⣰⡀⠱"}</p>
                                    <p>{"⠀⠀⠀⢹⣿⣿⡄⠀⠀⠀⠀⠀⠀⡀⡀⠀⠀⠀⠀⠀⠀⢀⣶⣿⣿⡟⠁"}</p>
                                    <p>{"⠀⠀⠘⣸⢿⣿⣿⣦⡀⠀⠀⠀⠀⠠⠄⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣇"}</p>
                                    <p>{"⠀⠀⠀⠉⠞⠿⠛⠿⠿⢶⣄⠀⠀⠀⠀⠀⠀⠀⣠⡾⠿⠿⣿⣿⡿⠅⠀⠀⠀"}</p>
                                    <p>{"⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣶⣤⣤⣤⡴⠊⠀⡧⠀⠀⣿⣿⣇"}</p>
                                    <p>{"⠀⠀⠀⠀⠀⠀⠀⠀⣀⡞⠛⠿⣿⣿⠟⠋⠀⠀⠀⠱⣀⠈⣿⣿⡁"}</p>
                                    <p>{"⠀⠀⠀⠀⠀⢠⡠⠔⠋⠀⠀⠀⠈⠁⠀⠀⠀⠀⠀⠀⠙⠲⣿⣧"}</p>
                                    <p>{"⢀⠔⠒⠀⠉⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡿⠀⠉⠚⠤⢔"}</p>
                                </div>
                                <div class="w-full my-2">
                            
                                </div>
                            </div>
                        </WindowRoot>

                        <WindowRoot ..window_props_lain>
                            <img class="w-full h-full bg-black" src="https://fauux.neocities.org/16c.gif" />
                        </WindowRoot>
                    </DesktopRoot>
                </main>
                <NavbarRoot />
            </div>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}

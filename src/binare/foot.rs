use yew::prelude::*;
use crate::system::WindowLucyRoot;
use crate::system::PropsWindowLucy;

#[component]
pub fn Foot() -> Html {
    let outputs = use_state(|| Vec::<String>::new());
    let output = use_state(|| html! {<></>});

    let window_props_terminal = yew::props! {
        PropsWindowLucy {
            name_window: "Terminal".to_string(),
            style_custom: "w-[600px] z-0".to_string(),
            sub_style: "bg-black overflow-y-auto overflow-x-hidden px-2 h-full text-white",
        }
    };


    html!{
        <WindowLucyRoot ..window_props_terminal>
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
                    <div class="my-2">
                        <div class="flex items-center">
                            <span class="text-pink-600">{"╭─"}</span>
                            <span class="text-pink-600">{"proxy"}</span>
                            <span>{"@"}</span>
                            <span class="text-pink-800">{"Skynet"}</span>
                            <span class="mx-1">{"in"}</span>
                            <span class="text-pink-600">{"/home"}</span>
                        </div>
                        <div class="flex items-center gap-1">
                            <span class="text-pink-600">{"╰─λ"}</span>
                            <input class="bg-transparent border-0 w-full outline-none"/>
                        </div>
                    </div>
                </div>
            </div>
        </WindowLucyRoot>
    }
}

use yew::prelude::*;
use web_sys::{
    HtmlInputElement,
    KeyboardEvent,
    console,
};
use crate::system::WindowLucyRoot;
use crate::system::PropsWindowLucy;

#[component]
pub fn Foot() -> Html {
    let output_history = use_state(|| Vec::<String>::new());
    let input_value = use_state(|| String::new());


    let window_props_terminal = yew::props! {
        PropsWindowLucy {
            name_window: "Terminal".to_string(),
            style_custom: "w-[600px] z-0".to_string(),
            sub_style: "bg-black overflow-y-auto overflow-x-hidden max-h-[500px] px-2 h-full text-white",
        }
    };

    let oninput = {
        let input_value = input_value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let onsubmit = {
        let output_history = output_history.clone();
        let input_value = input_value.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if input_value.is_empty() {
                return;
            }

            let mut new_line = (*output_history).clone();
            new_line.push(input_value.to_string());

            output_history.set(new_line);
            input_value.set(String::new());
        })
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

                <div>
                    { for output_history.iter().map(|line| html! {
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
                                    <p class="w-full text-white">{line.clone()}</p>
                                </div>
                            </div>
                        </div>
                    }) }
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
                        <form {onsubmit} class="flex items-center gap-1">
                            <span class="text-pink-600">{"╰─λ"}</span>
                            <input value={(*input_value).clone()} {oninput} autofocus=true class="bg-transparent border-0 w-full outline-none"/>
                        </form>
                    </div>
                </div>
            </div>
        </WindowLucyRoot>
    }
}

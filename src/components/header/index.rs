use yew::prelude::*;

#[component]
pub fn HeaderRoot() -> Html {

    html! {
        <header class="w-full h-fit p-0">
            <div class="bg-pink-300 text-black flex items-center relative justify-start gap-2 max-h-[30px]">
                <div class="py-0 flex items-center justify-center">
                    <button class="border-0">
                        <img class="w-[70px] max-h-[30px]" src="https://fauux.neocities.org/FloatingScreen.gif" />
                    </button>
                </div>
                <nav class="py-2">
                    <ul class="text-base font-bold flex items-center gap-5">
                        <li>{"Status"}</li>
                        <li>{"Discord"}</li>
                        <li>{"About"}</li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}

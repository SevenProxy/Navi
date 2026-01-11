use yew::prelude::*;

#[component]
pub fn NavbarRoot() -> Html {

    html!{
        <footer class="fixed bottom-1 left-0 w-full max-h-[40px]">
            <div class="w-full text-pink-300 h-full border-2 border-solid border-pink-300">
                <div class="flex items-center justify-between px-2 text-lg font-bold">
                    <div>
                        <p>{"Creator: 7proxy"}</p>
                    </div>
                    <span class="h-[40px] w-[2px] bg-pink-300" />
                    <div>
                        <p></p>
                    </div>
                    <span class="h-[40px] w-[2px] bg-pink-300"/>
                    <div>
                        <p>{"Languagem: BR"}</p>
                    </div>
                    <span class="h-[40px] w-[2px] bg-pink-300"/>
                    <div>
                        <p>{"22:00:40"}</p>
                    </div>
                </div>
            </div>
        </footer>
    }
}

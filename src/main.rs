mod components;

use yew::prelude::*;
use components::{
    header::HeaderRoot,
    navbar::NavbarRoot,
    desktop::DesktopRoot,
    window::WindowRoot,
};


#[component]
fn App() -> Html {

    html!{
        <div class="bg-black h-screen overflow-hidden">
            <div class="h-full">
                <HeaderRoot />
                <main class="h-full">
                    <DesktopRoot>
                        <WindowRoot />
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

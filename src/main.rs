mod components;
mod system;

use yew::prelude::*;
use components::{
    header::HeaderRoot,
    navbar::NavbarRoot,
    desktop::DesktopRoot,
    window::{
        WindowRoot,
        PropsWindow,
    },
    bin,
};

use system::Htop;


#[component]
fn App() -> Html {

    html!{
        <div class="bg-black h-screen overflow-hidden">
            <div class="h-full">
                <HeaderRoot />
                <main class="h-full">
                    <DesktopRoot>
                        <Htop/>
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

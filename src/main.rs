mod components;
mod system;

use yew::prelude::*;
use components::{
    header::HeaderRoot,
    navbar::NavbarRoot, 
    bin,
};

use system::{
    Htop,
    PantheonDesktopRoot,
};


#[component]
fn App() -> Html {

    html!{
        <div class="bg-black h-screen overflow-hidden">
            <div class="h-full">
                <HeaderRoot />
                <main class="h-full">
                    <PantheonDesktopRoot>
                        <Htop/>
                    </PantheonDesktopRoot>
                </main>
                <NavbarRoot />
            </div>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}

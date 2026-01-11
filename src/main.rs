mod components;

use yew::prelude::*;
use components::{
    header::HeaderRoot,
    navbar::NavbarRoot,
};


#[component]
fn App() -> Html {

    html!{
        <div class="bg-black h-screen overflow-hidden">
            <div>
                <HeaderRoot />
                <main>
                </main>
                <NavbarRoot />
            </div>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}

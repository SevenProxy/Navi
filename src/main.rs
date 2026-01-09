use yew::prelude::*;

#[component]
fn App() -> Html {

    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html!{
        <div>
            <button {onclick}>{ "+1" }</button>
            <p class="bg-red-600 text-red-600">{ *counter }</p>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}

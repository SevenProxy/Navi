use yew::prelude::*;

#[component]
pub fn WindowRoot() -> Html {
    let position = use_state(|| (100, 100));
    let dragging = use_state(|| false);
    let last_mouse = use_state(|| (0, 0));

    let on_mousedown = {
        let dragging = dragging.clone();
        let last_mouse = last_mouse.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            dragging.set(true);
            last_mouse.set((e.client_x(), e.client_y()))
        })
    };

    let on_mouseup = {
        let dragging = dragging.clone();
        Callback::from(move |_| {
            dragging.set(false);
        })
    };

    let on_mousemove = {
        let dragging = dragging.clone();
        let position = position.clone();
        let last_mouse = last_mouse.clone();

        Callback::from(move |e: MouseEvent| {
            if *dragging {
                let (lx, ly) = *last_mouse;
                let dx = e.client_x() - lx;
                let dy = e.client_y() - ly;

                let (x, y) = *position;
                position.set((x + dx, y + dy));
                last_mouse.set((e.client_x(), e.client_y()));
            }
        })
    };

    let style = format!(
        "left: {}px; top: {}px;",
        position.0, position.1
    );

    html!{
        <section
            class="font-bold text-base absolute z-10 w-[400px] h-[500px] bg-back border-2 border-solid border-pink-300"
            onmousemove={on_mousemove}
            onmouseup={on_mouseup}
            style={style}
        >
            <div class="h-[30px] bg-pink-300 cursor-move flex items-center justify-between gap-2 py-2 px-2" onmousedown={on_mousedown}>
                <div class="flex items-center gap-2">
                    <span class="h-[25px] w-[25px] border-[7px] border-solid border-pink-600 bg-pink-300"/>
                    <span class="h-[25px] w-[25px] border-[7px] border-solid border-pink-600 bg-pink-300"/>
                </div>
                <div class="flex items-center justify-between gap-2 w-full">
                    <span class="h-2 w-full bg-pink-600"/>
                    <p class="whitespace-nowrap">{"[ Minha janela ]"}</p>
                    <span class="h-2 w-full bg-pink-600"/>
                </div>
                <button class="w-[50px] h-[25px] border-[7px] border-solid border-pink-600 bg-pink-300">
                </button>
            </div>
            <div class="px-5 h-full text-white">
                {"contextoooooooo"}
            </div>
        </section>
    }
}

use yew::prelude::*;
use gloo::timers::callback::Timeout;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsStart {
    pub state: UseStateHandle<bool>,
}

#[derive(Clone)]
pub struct BootLine {
    text: String,
    ok: bool,
}

fn run_started_log() -> Vec<String> {
    let lines = vec![
        "Iniciando...".into(),
        "Feito...".into(),
        "Compilando...".into(),
    ];

    lines
}

#[component]
pub fn StartRoot(props: &PropsStart) -> Html {
    let lines = use_state(|| Vec::<BootLine>::new());
    let timeout_ref = use_mut_ref(|| Vec::<Timeout>::new());

    {
        let lines = lines.clone();
        let timeout_ref = timeout_ref.clone();

        use_effect_with((), move |_|  {
            let delay_step = 600;

            for (i, v) in run_started_log().into_iter().enumerate() {
                let lines = lines.clone();
                let v = v.clone();

                let time_closure = Timeout::new(i as u32 * delay_step, move || {
                    lines.set({
                        let mut current = (*lines).clone();
                        current.push(BootLine {
                            text: v,
                            ok: true,
                        });

                        current
                    });
                });

                timeout_ref.borrow_mut().push(time_closure);
            }

            move || {
                timeout_ref.borrow_mut().clear();
            }
        });
    }

    html! {
        <div class="z-50 h-screen w-full overflow-hidden bg-black text-white">
            <div class="w-full h-full">
                { 
                    for lines.iter().map(|m| html! {
                        <div class="flex items-center gap-2">
                            <p>{"OK"}</p>
                            <p>{&m.text}</p>
                            <span class="animate-pulse">{"▮"}</span>
                        </div>
                    })
                }
            </div>
        </div>
    }
}

use yew::prelude::*;
use std::rc::Rc;
use gloo::timers::callback::Timeout;

#[derive(Properties, PartialEq, Clone)]
pub struct PropsStart {
    pub state: UseStateHandle<bool>,
}

#[derive(Clone, PartialEq)]
pub struct BootLine {
    text: String,
    ok: bool,
}


#[derive(Clone, PartialEq)]
struct BootState {
    lines: Vec<BootLine>,
}

enum BootAction {
    Push(BootLine),
    Clear,
}

impl Reducible for BootState {
    type Action = BootAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            BootAction::Push(line) => {
                let mut lines = self.lines.clone();
                lines.push(line);

                Self { lines }.into()
            }

            BootAction::Clear => Self { lines: Vec::new() }.into(),
        }
    }
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
    let states = use_reducer(|| BootState {
        lines: Vec::new(),
    });
    let timeout_ref = use_mut_ref(|| Vec::<Timeout>::new());

    {
        let states = states.dispatcher();
        let props = props.clone();
        let timeout_ref = timeout_ref.clone();

        use_effect_with((), move |_|  {
            let delay_step = 600;

            for (i, v) in run_started_log().into_iter().enumerate() {
                let dispatch = states.clone();
                let v = v.clone();

                let time_closure = Timeout::new(i as u32 * delay_step, move || {
                    dispatch.dispatch(BootAction::Push(BootLine {
                        text: v,
                        ok: true,
                    }));
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
                    for states.lines.iter().map(|m| html! {
                        <div class="flex items-center gap-2">
                            <div class="text-bold flex items-center gap-2">
                                <span>{"["}</span>
                                <p class="text-green-600">{"OK"}</p>
                                <span>{"]"}</span>
                            </div>
                            <p class="text-zinc-300">{&m.text}</p>
                        </div>
                    })
                }
                <span class="animate-ping">{"▮"}</span>
            </div>
        </div>
    }
}

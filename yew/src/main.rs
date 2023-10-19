use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let task_done = use_state(|| false);
    let onclick = {
        let task_status = task_done.clone();
        move |_| {
            let new_status = !*task_status;
            task_status.set(new_status);
        }
    };

    html! {
        <div>
            <p {onclick} style="cursor: pointer;">{if *task_done { "[X] "} else {"[o] "}}{ "A task" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

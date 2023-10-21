use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyData {
    name: String,
    fav_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello, world!" }</h1>
            <LogAndClass />
            <Conditionals />
        </>
    }
}

#[function_component(Conditionals)]
pub fn conditionals() -> Html {
    let class = "class_name";
    let message_some = Some("This is a message that contains stuff");
    let message_none: Option<&str> = None;

    html! {
        <>
            if class == "class_name" {
                <p>{"Conditional presence"}</p>
            } else {
                <p>{"Condition did not pass"}</p>
            }
            if let Some(result) = message_some {
                <p>{result}</p>
            }
            if let Some(result) = message_none {
                <p>{result}</p>
            } else {
                <p>{"No message here"}</p>
            }
        </>
    }
}

#[function_component(LogAndClass)]
fn log_and_class() -> Html {
    let d = MyData {
        name: "Pol".to_owned(),
        fav_language: "Rust".to_owned(),
    };
    let my_class = "useful_class";

    log!(serde_json::to_string_pretty(&d).unwrap());
    html! {
        <div class={my_class}>
            <p>{"Open console to see Gloo's log"}</p>
        </div>
    }
}

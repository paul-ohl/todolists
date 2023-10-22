use gloo::console::log;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

mod components;

use components::atoms::main_title::MainTitle;

#[derive(Serialize, Deserialize)]
struct MyData {
    name: String,
    fav_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));
    html! {
        <>
            <MainTitle title="Hello there!" on_load={main_title_load} />
            // <LogAndClass />
            // <Conditionals />
            // <Loops />
            // <StylistComponent />
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

#[function_component(Loops)]
fn loops() -> Html {
    let tasks = vec![
        "clean the dishes",
        "make the bed",
        "code your entire life into nvim",
    ];
    let numbers = vec![3, 5, 11];
    html! {
        <ol>
            { list_to_html(tasks) }
            <ul>
                { list_to_html(numbers) }
            </ul>
        </ol>
    }
}

fn list_to_html<T>(list: Vec<T>) -> Html
where
    T: Display,
{
    list.iter()
        .map(|e| {
            html! {
                <li>
                    { e }
                </li>
            }
        })
        .collect()
}

#[styled_component(StylistComponent)]
fn stylist_component() -> Html {
    let stylesheet = style!(
        r#"
        h1, h2, h3 {
            color: #8fbcbb;
        }
        p {
            color: #88c0d0;
        }
        "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <h2>{ "Hello guys! I'm styled" }</h2>
            <p>{ "Other styled text" }</p>
        </div>
    }
}

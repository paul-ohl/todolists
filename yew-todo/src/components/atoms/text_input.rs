use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = Callback::from(|e: Event| {
        let value = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        log!(value);
    });
    html! {
        <input text="name" name={ props.name.clone() } placeholder={ props.name.clone() } onchange={ onchange } />
    }
}

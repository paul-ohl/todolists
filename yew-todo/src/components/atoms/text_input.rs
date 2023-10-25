use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = Callback::from(|e: Event| {
        let value = e.target_unchecked_into::<HtmlInputElement>().value();
        log!("onchange: ", value);
    });
    let oninput = Callback::from(|e: InputEvent| {
        let value = e.target_unchecked_into::<HtmlInputElement>().value();
        log!("oninput: ", value);
    });
    html! {
        <>
            <input text="onchange"
                name={ props.name.clone() }
                placeholder={ props.name.clone() }
                onchange={ onchange }
            />
            <input text="oninput"
                name={ props.name.clone() }
                placeholder={ "oninput" }
                oninput={ oninput }
            />
        </>
    }
}

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    html! {
        <input text="name" name={ props.name.clone() } placeholder={ props.name.clone() } />
    }
}

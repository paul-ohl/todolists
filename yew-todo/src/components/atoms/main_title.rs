use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub on_load: Callback<String>,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    props.on_load.emit("I loaded".to_owned());
    html! {
        <h1>{ &props.title }</h1>
    }
}

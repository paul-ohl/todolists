use yew::prelude::*;

use crate::components::atoms::{button::Button, text_input::TextInput};

// #[derive(Properties, PartialEq)]
// pub struct Props {
//     pub title: String,
//     pub on_load: Callback<String>,
// }

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html! {
        <form>
            <TextInput name="username" />
            <Button label="Submit!" />
        </form>
    }
}

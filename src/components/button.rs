use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_click: Callback<MouseEvent>,
    pub text: String,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    html! {
        <button onclick={props.on_click.clone()}>{ props.text.clone() }</button>
    }
}

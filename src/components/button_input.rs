use ::yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub is_revealed: bool,
    pub on_reveal: Callback<MouseEvent>,
    pub on_new_pokemon: Callback<MouseEvent>,
}

#[function_component]
pub fn ButtonInput(
    Props {
        is_revealed,
        on_reveal,
        on_new_pokemon,
    }: &Props,
) -> Html {
    if *is_revealed {
        html! {<button type="button" onclick={on_new_pokemon.clone()}>{"Next"}</button>}
    } else {
        html! {<button type="button" onclick={on_reveal.clone()}>{"Reveal"}</button>}
    }
}

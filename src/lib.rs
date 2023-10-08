use crate::components::guesser::Guesser;
use yew::prelude::*;

mod components;
mod models;
mod util;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <h1>{"Who's That Pokémon?"}</h1>
            <Guesser/>
        </>
    }
}

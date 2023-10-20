use crate::components::{guesser::Guesser, title::Title};
use yew::prelude::*;

mod components;
mod models;
mod util;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <Title/>
            <Guesser/>
        </>
    }
}

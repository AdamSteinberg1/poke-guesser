use crate::components::{guesser::Guesser, title::Title};
use yew::prelude::*;

mod components;
mod models;
mod util;

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<img class = "loading" src="assets/question_mark.png"/>};
    html! {
        <>
            <Title/>
            <Suspense {fallback}>
                <Guesser/>
            </Suspense>
        </>
    }
}

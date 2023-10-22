use crate::{
    components::starburst::Starburst,
    util::{fetch_rand_pokemons, shuffle},
};
use ::yew::prelude::*;
use futures::FutureExt;
use gloo::console::log;
use yew_hooks::{use_async_with_options, use_counter, UseAsyncOptions};

#[function_component]
pub fn Guesser() -> Html {
    let counter = use_counter(0);
    let is_name_revealed = use_state_eq(|| false);
    let pokemons = use_async_with_options(
        fetch_rand_pokemons().then(|pokemons| async { pokemons.map_err(|e| e.to_string()) }),
        UseAsyncOptions::enable_auto(),
    );

    let on_reveal = {
        let is_name_revealed = is_name_revealed.clone();
        Callback::from(move |_| {
            is_name_revealed.set(true);
        })
    };

    let on_new_pokemon = {
        let is_name_revealed = is_name_revealed.clone();
        let counter = counter.clone();
        let pokemons = pokemons.clone();
        Callback::from(move |_| {
            if let Some(data) = pokemons.data.as_ref() {
                is_name_revealed.set(false);
                if (*counter as usize) < data.len() - 1 {
                    counter.increase();
                } else {
                    counter.reset();
                    pokemons.update(shuffle(data.clone()));
                }
            }
        })
    };

    pokemons
        .data
        .as_ref()
        .map(|data| {
            let pokemon = data[*counter as usize].clone();
            html! {
                <>
                    <div class="pokemon-wrapper">
                        <img src={pokemon.image.clone()}/>
                        <Starburst/>
                    </div>
                    <svg class="pokemon-name" xmlns="http://www.w3.org/2000/svg">
                        <text x="50%" y="50%" visibility={is_name_revealed.then_some("visible").unwrap_or("hidden")}>
                          {format!("{}: {}", pokemon.id, pokemon.name)}
                        </text>
                    </svg>
                    if *is_name_revealed {
                        <button type="button" onclick={on_new_pokemon.clone()}>{"Next"}</button>
                    } else {
                        <button type="button" onclick={on_reveal.clone()}>{"Reveal"}</button>
                    }
                </>
            }
        })
        .or_else(|| {
            pokemons.error.as_ref().map(|e| {
                log!("Error when fetching pokemon data:\n{:?}", e);
                html! {<p>{"error :("}</p>}
            })
        })
        .unwrap_or(html! {<img class = "loading" src="assets/question_mark.png"/>})
}

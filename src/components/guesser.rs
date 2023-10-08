use crate::{
    models::pokemon::Pokemon,
    util::{fetch_rand_pokemons, shuffle},
};
use ::yew::prelude::*;
use futures::FutureExt;
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
                if (*counter as usize) < data.len() - 1 {
                    counter.increase();
                } else {
                    counter.reset();
                    pokemons.update(shuffle(data.clone()));
                }
                is_name_revealed.set(false);
            }
        })
    };

    let guesser_core = |pokemon: Pokemon| {
        html! {
            <>
                <img class={classes!("pokemon-image")} src={pokemon.image.clone()}/>
                <br/>
                if *is_name_revealed {
                    <h3>{format!("{}: {}", pokemon.id, pokemon.name)}</h3>
                    <button type="button" onclick={on_new_pokemon.clone()}>{"Next"}</button>
                } else {
                    <button type="button" onclick={on_reveal.clone()}>{"Reveal"}</button>
                }
            </>
        }
    };

    match (pokemons.error.as_ref(), pokemons.data.as_ref()) {
        (_, Some(data)) => guesser_core(data[*counter as usize].clone()),
        (Some(_), _) => html! {<p>{"error :("}</p>},
        _ => html! {<p>{"loading"}</p>},
    }
}

use crate::{
    components::{pokemon_image::PokemonImage, pokemon_label::PokemonLabel},
    models::settings::Settings,
    util::fetch_pokemons,
};
use ::yew::prelude::*;
use gloo::console::log;
use yew::suspense::use_future;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub settings: Settings,
}

#[function_component]
pub fn Guesser(Props { settings }: &Props) -> HtmlResult {
    let is_name_revealed = use_state_eq(|| false);
    let pokemons = use_future(fetch_pokemons)?;

    let on_reveal = {
        let is_name_revealed = is_name_revealed.clone();
        Callback::from(move |_| {
            is_name_revealed.set(true);
        })
    };

    let on_new_pokemon = {
        let is_name_revealed = is_name_revealed.clone();
        Callback::from(move |_| {
            is_name_revealed.set(false);
        })
    };

    match *pokemons {
        Err(ref e) => {
            log!(format!("Error when fetching pokemon data:\n{:?}", e));
            Ok(html! {<p><span>{"An error has occurred. 😢"}</span></p>})
        }
        Ok(ref pokemons) => {
            let pokemon = if *is_name_revealed {
                pokemons.current()
            } else {
                pokemons.next()
            };
            let next_pokemon = pokemons.peek();
            Ok(html! {
                <>
                    <link rel="prefetch" href={next_pokemon.image.clone()} as="image" />
                    <PokemonImage silhouette={settings.silhouette && !(*is_name_revealed)} image={pokemon.image.clone()}/>
                    <PokemonLabel is_revealed={*is_name_revealed} name={pokemon.name.clone()} id={pokemon.id}/>
                    if *is_name_revealed {
                        <button type="button" onclick={on_new_pokemon.clone()}>{"Next"}</button>
                    } else {
                        <button type="button" onclick={on_reveal.clone()}>{"Reveal"}</button>
                    }
                </>
            })
        }
    }
}

use crate::{
    components::{
        button_input::ButtonInput, pokemon_image::PokemonImage, pokemon_label::PokemonLabel,
        text_input::TextInput,
    },
    models::settings::Settings,
    util::fetch_pokemons,
};
use ::yew::prelude::*;
use futures::TryFutureExt;
use gloo::console::error;
use std::rc::Rc;
use yew::suspense::use_future;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub settings: Settings,
}

#[function_component]
pub fn Guesser(Props { settings }: &Props) -> HtmlResult {
    let is_name_revealed = use_state_eq(|| false);
    let pokemons = use_future(|| fetch_pokemons().map_ok(Rc::new))?;

    let pokemons = match *pokemons {
        Ok(ref pokemons) => Rc::clone(pokemons),
        Err(ref e) => {
            error!(format!("Error when fetching pokemon data:\n{:?}", e));
            return Ok(html! {<p><span>{"An error has occurred. 😢"}</span></p>});
        }
    };

    let on_reveal = {
        let is_name_revealed = is_name_revealed.clone();
        Callback::from(move |_| {
            is_name_revealed.set(true);
        })
    };

    let on_new_pokemon = {
        let is_name_revealed = is_name_revealed.clone();
        let pokemons = Rc::clone(&pokemons);
        Callback::from(move |_| {
            is_name_revealed.set(false);
            pokemons.next();
        })
    };

    let pokemon = pokemons.current();
    let next_pokemon = pokemons.peek();
    Ok(html! {
        <>
            <link rel="prefetch" href={next_pokemon.image.clone()} as="image" />
            <PokemonImage silhouette={settings.silhouette && !(*is_name_revealed)} image={pokemon.image.clone()}/>
            if settings.text_entry {
                if *is_name_revealed {
                    <PokemonLabel is_revealed={*is_name_revealed} name={pokemon.name.clone()} id={pokemon.id}/>
                }
                <TextInput
                    is_revealed={*is_name_revealed}
                    {on_reveal}
                    {on_new_pokemon}
                    name={pokemon.name.clone()}/>
            } else {
                <PokemonLabel is_revealed={*is_name_revealed} name={pokemon.name.clone()} id={pokemon.id}/>
                <ButtonInput
                    is_revealed={*is_name_revealed}
                    {on_reveal}
                    {on_new_pokemon}/>
            }
        </>
    })
}

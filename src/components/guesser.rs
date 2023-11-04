use crate::{components::starburst::Starburst, models::settings::Settings, util::fetch_pokemons};
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

    Ok(match *pokemons {
        Err(ref e) => {
            log!(format!("Error when fetching pokemon data:\n{:?}", e));
            html! {<p>{"An error has occurred. 😢"}</p>}
        }
        Ok(ref pokemons) => {
            let pokemon = if *is_name_revealed {
                pokemons.next()
            } else {
                pokemons.peek()
            };
            html! {
                <>
                    <div class="pokemon-wrapper">
                        <img src={pokemon.image.clone()}
                            style={format!("filter: {} drop-shadow(-0.2rem 0.2rem black)",
                                (settings.silhouette && !(*is_name_revealed))
                                    .then_some("url(assets/silhouette_filter.svg#filter)")
                                    .unwrap_or_default())}/>
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
        }
    })
}

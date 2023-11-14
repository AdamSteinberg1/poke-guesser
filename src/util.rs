use std::str::FromStr;

use crate::models::{
    pokemon::{Pokemon, PokemonType},
    pokemon_list::PokemonList,
};
use anyhow::{ensure, Result};
use gloo::net::http::Request;
use itertools::Itertools;

pub async fn fetch_pokemons() -> Result<PokemonList> {
    let url =
        "https://m.bulbapedia.bulbagarden.net/wiki/List_of_Pokémon_by_National_Pokédex_number";
    let response = Request::get(url).send().await?.text().await?;
    let pokemons = parse_pokemons(&response);
    ensure!(!pokemons.is_empty(), "No pokemon parsed from HTML");
    Ok(PokemonList::new(pokemons))
}

fn parse_pokemons(html: &str) -> Vec<Pokemon> {
    html.split("<tr style=\"background:#FFF\">\n")
        .filter_map(|row| {
            let (id, image, name, primary_type, secondary_type) =
                row.lines().take(5).collect_tuple()?;

            let id = id.split(['<', '>', '#']).nth(3)?.parse::<u16>().ok()?;
            if id <= 0 {
                return None;
            }

            let name = name.split(['<', '>']).nth(4)?.into();

            let image = image
                .split('"')
                .nth(7)?
                .rsplit_once("/")
                .map(|(s, _)| String::from("https:") + &s.replace("thumb/", ""))?;

            let primary_type = primary_type
                .split(['<', '>'])
                .nth(6)
                .and_then(|t| PokemonType::from_str(t).ok())?;

            let secondary_type = secondary_type
                .split(['<', '>'])
                .nth(6)
                .and_then(|t| PokemonType::from_str(t).ok());

            Some(Pokemon {
                name,
                image,
                primary_type,
                secondary_type,
            })
        })
        .collect()
}

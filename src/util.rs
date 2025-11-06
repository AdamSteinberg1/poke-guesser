use crate::models::{
    pokemon::{Pokemon, PokemonType},
    pokemon_list::PokemonList,
};
use anyhow::{ensure, Result};
use gloo::net::http::Request;
use itertools::Itertools;
use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

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

            let id = without_tags(id)?.strip_prefix("#")?.parse::<u16>().ok()?;
            if id <= 0 {
                return None;
            }

            let name = without_tags(name)?;

            let image = extract_src(image)?
                .rsplit_once("/")
                .map(|(s, _)| s.replace("thumb/", ""))?;

            let primary_type =
                without_tags(primary_type).and_then(|t| PokemonType::from_str(&t).ok())?;

            let secondary_type =
                without_tags(secondary_type).and_then(|t| PokemonType::from_str(&t).ok());

            Some(Pokemon {
                name,
                image,
                primary_type,
                secondary_type,
            })
        })
        .collect()
}

fn without_tags(html: &str) -> Option<String> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]*>").unwrap());
    let parsed = RE.replace_all(html, "").trim().to_string();
    Some(parsed).filter(|s| !s.is_empty())
}

fn extract_src(html: &str) -> Option<String> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"src="([^"]+)""#).unwrap());
    let src = RE.captures(html)?.get(1)?.as_str().to_string();
    Some(src)
}

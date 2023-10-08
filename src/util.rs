use crate::models::pokemon::Pokemon;
use anyhow::Result;
use gloo::net::http::Request;
use itertools::Itertools;

pub fn shuffle<T>(vec: Vec<T>) -> Vec<T> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut vec = vec;
    let mut rng = thread_rng();
    vec.shuffle(&mut rng);
    vec
}

pub async fn fetch_rand_pokemons() -> Result<Vec<Pokemon>> {
    fetch_pokemons().await.map(shuffle)
}

async fn fetch_pokemons() -> Result<Vec<Pokemon>> {
    let url =
        "https://m.bulbapedia.bulbagarden.net/wiki/List_of_Pokémon_by_National_Pokédex_number";
    let response = Request::get(&url).send().await?.text().await?;
    Ok(parse_pokemons(&response))
}

fn parse_pokemons(text: &str) -> Vec<Pokemon> {
    use scraper::Html;
    use scraper::Selector;

    Html::parse_document(text)
        .select(&Selector::parse("table.roundy > tbody > tr").unwrap())
        .filter_map(|row| {
            let (id, name) = row
                .select(&Selector::parse("td").unwrap())
                .step_by(2)
                .take(2)
                .filter_map(|cell| cell.text().next().map(String::from))
                .collect_tuple()?;
            let image = row
                .select(&Selector::parse("img").unwrap())
                .filter_map(|img| {
                    img.value()
                        .attr("src")
                        .and_then(|src| src.rsplit_once("/"))
                        .map(|(s, _)| format!("https:{}", s.replace("thumb/", "")))
                })
                .next()?;

            for field in [&id, &name, &image] {
                if field.is_empty() {
                    return None;
                }
            }

            let id = id.strip_prefix("#")?.parse().ok()?;
            if id <= 0 {
                return None;
            }

            Some(Pokemon { id, name, image })
        })
        .collect()
}

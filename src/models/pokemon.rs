use strum::{Display, EnumString};

#[derive(Clone)]
pub struct Pokemon {
    pub id: u16,
    pub name: String,
    pub image: String,
    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,
}

#[derive(Clone, Debug, Display, PartialEq, EnumString)]
pub enum PokemonType {
    Normal,
    Fire,
    Fighting,
    Water,
    Flying,
    Grass,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
}

impl PokemonType {
    pub fn color(&self) -> String {
        match self {
            Self::Normal => "#A8A878",
            Self::Fire => "#F08030",
            Self::Fighting => "#C03028",
            Self::Water => "#6890F0",
            Self::Flying => "#A890F0",
            Self::Grass => "#78C850",
            Self::Poison => "#A040A0",
            Self::Electric => "#F8D030",
            Self::Ground => "#E0C068",
            Self::Psychic => "#F85888",
            Self::Rock => "#B8A038",
            Self::Ice => "#98D8D8",
            Self::Bug => "#A8B820",
            Self::Dragon => "#7038F8",
            Self::Ghost => "#705898",
            Self::Dark => "#705848",
            Self::Steel => "#B8B8D0",
            Self::Fairy => "#EE99AC",
        }
        .into()
    }
}

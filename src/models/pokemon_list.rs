use super::pokemon::Pokemon;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::Cell;
use std::cell::RefCell;
use std::iter;
use std::vec;

type GenerateIndices = fn(usize) -> vec::IntoIter<usize>;
type Indices = RefCell<
    iter::Peekable<iter::FlatMap<iter::Repeat<usize>, vec::IntoIter<usize>, GenerateIndices>>,
>;

pub struct PokemonList {
    pokemons: Vec<Pokemon>,
    indices: Indices,
    current_index: Cell<usize>,
}

impl PokemonList {
    pub fn new(pokemons: impl IntoIterator<Item = Pokemon>) -> Self {
        let pokemons: Vec<_> = pokemons.into_iter().collect();
        assert!(!pokemons.is_empty(), "PokemonList cannot be created empty");

        let generate_indices: GenerateIndices = |len: usize| {
            let mut indices: Vec<_> = (0..len).collect();
            indices.shuffle(&mut thread_rng());
            indices.into_iter()
        };

        let mut indices = iter::repeat(pokemons.len())
            .flat_map(generate_indices)
            .peekable();
        let current_index = indices
            .next()
            .expect("Iterator should always have a next because it is infinite");
        Self {
            pokemons,
            current_index: Cell::new(current_index),
            indices: RefCell::new(indices),
        }
    }

    pub fn next(&self) -> &Pokemon {
        let index = self
            .indices
            .borrow_mut()
            .next()
            .expect("Iterator should always have a next because it is infinite");
        self.current_index.set(index);
        &self.pokemons[index]
    }

    pub fn peek(&self) -> &Pokemon {
        let index = self
            .indices
            .borrow_mut()
            .peek()
            .cloned()
            .expect("Iterator should always have a next because it is infinite");
        &self.pokemons[index]
    }

    pub fn current(&self) -> &Pokemon {
        &self.pokemons[self.current_index.get()]
    }
}

use super::pokemon::Pokemon;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::RefCell;
use std::iter;
use std::vec;

type GenerateIndices = fn(usize) -> vec::IntoIter<usize>;
type Indices = RefCell<
    iter::Peekable<
        iter::FlatMap<iter::Cycle<iter::Once<usize>>, vec::IntoIter<usize>, GenerateIndices>,
    >,
>;

pub struct PokemonList {
    pokemons: Vec<Pokemon>,
    indices: Indices,
}

impl PokemonList {
    pub fn new(pokemons: impl IntoIterator<Item = Pokemon>) -> Self {
        let pokemons: Vec<_> = pokemons.into_iter().collect();

        let generate_indices: GenerateIndices = |len: usize| {
            let mut indices: Vec<_> = (0..len).collect();
            indices.shuffle(&mut thread_rng());
            indices.into_iter()
        };

        let indices = iter::once(pokemons.len())
            .cycle()
            .flat_map(generate_indices)
            .peekable();
        Self {
            pokemons,
            indices: RefCell::new(indices),
        }
    }

    pub fn next(&self) -> Pokemon {
        let index = self
            .indices
            .borrow_mut()
            .next()
            .expect("Iterator should always have a next because it is infinite");
        self.pokemons[index].clone()
    }

    pub fn current(&self) -> Pokemon {
        let index = self
            .indices
            .borrow_mut()
            .peek()
            .cloned()
            .expect("Iterator should always have a next because it is infinite");
        self.pokemons[index].clone()
    }
}

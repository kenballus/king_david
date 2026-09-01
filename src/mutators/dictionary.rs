use {
    crate::Mutator,
    rand::{RngExt, rngs::SmallRng},
};

pub struct DictionaryMutator<'a> {
    rng: SmallRng,
    dictionary: &'a [Vec<u8>],
}

impl<'a> DictionaryMutator<'a> {
    #[must_use]
    pub fn new(rng: SmallRng, dictionary: &'a [Vec<u8>]) -> Self {
        Self { rng, dictionary }
    }
}

impl Mutator for DictionaryMutator<'_> {
    type Input = Vec<u8>;
    fn mutate(&mut self, input: &Self::Input) -> Self::Input {
        let word = &self.dictionary[self.rng.random_range(0..self.dictionary.len())];
        let mut result = Vec::with_capacity(input.len() + word.len());
        let insertion_pt = self.rng.random_range(0..=input.len());
        result.extend_from_slice(&input[..insertion_pt]);
        result.extend_from_slice(word);
        result.extend_from_slice(&input[insertion_pt..]);
        result
    }
}

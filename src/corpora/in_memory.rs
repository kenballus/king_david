use {
    crate::Corpus,
    rand::{RngExt, rngs::SmallRng},
};

pub struct InMemoryCorpus<I> {
    rng: SmallRng,
    corpus: Vec<I>,
}

impl<I> InMemoryCorpus<I> {
    #[must_use]
    pub fn new(rng: SmallRng, corpus: Vec<I>) -> Self {
        assert!(
            !corpus.is_empty(),
            "Corpus must be initialized with at least one element."
        );
        Self { rng, corpus }
    }
}

impl<I> Corpus for InMemoryCorpus<I> {
    type Input = I;
    fn select(&mut self) -> &Self::Input {
        let index = self.rng.random_range(0..self.corpus.len());
        &self.corpus[index]
    }

    fn add(&mut self, input: Self::Input) {
        self.corpus.push(input);
    }
}

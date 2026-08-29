use {
    crate::Mutator,
    rand::{RngExt, SeedableRng, rngs::SmallRng},
};

pub struct ByteInsertionMutator {
    rng: SmallRng,
}

impl ByteInsertionMutator {
    #[must_use]
    pub fn new(rng: SmallRng) -> Self {
        Self { rng }
    }
}

impl Mutator for ByteInsertionMutator {
    type Input = Vec<u8>;
    fn mutate(&mut self, input: &Self::Input) -> Self::Input {
        let mut result = Vec::with_capacity(input.len() + 1);
        let insertion_pt = self.rng.random_range(0..=input.len());
        result.extend_from_slice(&input[..insertion_pt]);
        result.push(self.rng.random::<u8>());
        result.extend_from_slice(&input[insertion_pt..]);
        result
    }
}

pub struct ByteDeletionMutator {
    rng: SmallRng,
}

impl ByteDeletionMutator {
    #[must_use]
    pub fn new(rng: SmallRng) -> Self {
        Self { rng }
    }
}

impl Mutator for ByteDeletionMutator {
    type Input = Vec<u8>;
    fn mutate(&mut self, input: &Self::Input) -> Self::Input {
        debug_assert!(
            !input.is_empty(),
            "Can't delete bytes when there's nothing to delete"
        );
        let mut result = Vec::with_capacity(input.len() - 1);
        let deletion_pt = self.rng.random_range(0..input.len());
        result.extend_from_slice(&input[..deletion_pt]);
        result.extend_from_slice(&input[(deletion_pt + 1)..]);
        result
    }
}

pub struct ByteReplacementMutator {
    rng: SmallRng,
}

impl ByteReplacementMutator {
    #[must_use]
    pub fn new(rng: SmallRng) -> Self {
        Self { rng }
    }
}

impl Mutator for ByteReplacementMutator {
    type Input = Vec<u8>;
    fn mutate(&mut self, input: &Self::Input) -> Self::Input {
        let mut result = input.clone();
        debug_assert!(
            !input.is_empty(),
            "Can't replace bytes when there's nothing to replace"
        );
        result[self.rng.random_range(0..input.len())] = self.rng.random::<u8>();
        result
    }
}

pub struct ByteMutator {
    rng: SmallRng,
    replacer: ByteReplacementMutator,
    deleter: ByteDeletionMutator,
    inserter: ByteInsertionMutator,
}

impl ByteMutator {
    #[must_use]
    pub fn new(mut rng: SmallRng) -> Self {
        let replacer = ByteReplacementMutator::new(SmallRng::from_rng(&mut rng));
        let deleter = ByteDeletionMutator::new(SmallRng::from_rng(&mut rng));
        let inserter = ByteInsertionMutator::new(SmallRng::from_rng(&mut rng));
        Self {
            rng,
            replacer,
            deleter,
            inserter,
        }
    }
}

impl Mutator for ByteMutator {
    type Input = Vec<u8>;
    fn mutate(&mut self, input: &Self::Input) -> Self::Input {
        if input.is_empty() {
            self.inserter.mutate(input)
        } else {
            match self.rng.random_range(0..3) {
                0 => self.replacer.mutate(input),
                1 => self.inserter.mutate(input),
                2 => self.deleter.mutate(input),
                _ => unreachable!(),
            }
        }
    }
}

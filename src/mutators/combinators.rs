use {
    crate::Mutator,
    rand::{RngExt, rngs::SmallRng},
};

pub struct RandomChoiceCombinatorMutator<M1, M2> {
    rng: SmallRng,
    probability: f64,
    m1: M1,
    m2: M2,
}

impl<M1, M2> RandomChoiceCombinatorMutator<M1, M2> {
    #[must_use]
    pub fn new(rng: SmallRng, probability: f64, m1: M1, m2: M2) -> Self {
        Self {
            rng,
            probability,
            m1,
            m2,
        }
    }
}

impl<I, M1: Mutator<Input = I>, M2: Mutator<Input = I>> Mutator
    for RandomChoiceCombinatorMutator<M1, M2>
{
    type Input = I;
    fn mutate(&mut self, input: &Self::Input) -> Self::Input {
        if self.rng.random_bool(self.probability) {
            self.m1.mutate(input)
        } else {
            self.m2.mutate(input)
        }
    }
}

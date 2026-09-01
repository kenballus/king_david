use crate::Executor;

pub struct DuplicatorExecutor<E1, E2> {
    e1: E1,
    e2: E2,
}

impl<I, O1, O2, E1: Executor<Input = I, Output = O1>, E2: Executor<Input = I, Output = O2>> Executor
    for DuplicatorExecutor<E1, E2>
{
    type Input = I;
    type Output = (O1, O2);

    fn run(&mut self, input: &Self::Input) -> Self::Output {
        (self.e1.run(input), self.e2.run(input))
    }
}

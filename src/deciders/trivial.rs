use {crate::Decider, std::marker::PhantomData};

#[derive(Default)]
pub struct AlwaysResult<I, O> {
    p1: PhantomData<I>,
    p2: PhantomData<O>,
}
impl<I, O> Decider for AlwaysResult<I, O> {
    type ExecutionOutput = O;
    type Input = I;
    fn is_result(&mut self, _: &Self::Input, _: &Self::ExecutionOutput) -> bool {
        true
    }
}

#[derive(Default)]
pub struct NeverResult<I, O> {
    p1: PhantomData<I>,
    p2: PhantomData<O>,
}
impl<I, O> Decider for NeverResult<I, O> {
    type ExecutionOutput = O;
    type Input = I;
    fn is_result(&mut self, _: &Self::Input, _: &Self::ExecutionOutput) -> bool {
        false
    }
}

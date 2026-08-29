use crate::Decider;

pub struct AndSplitDecider<
    I,
    O1,
    O2,
    D1: Decider<ExecutionOutput = O1, Input = I>,
    D2: Decider<ExecutionOutput = O2, Input = I>,
> {
    d1: D1,
    d2: D2,
}

impl<
    I,
    O1,
    O2,
    D1: Decider<Input = I, ExecutionOutput = O1>,
    D2: Decider<Input = I, ExecutionOutput = O2>,
> AndSplitDecider<I, O1, O2, D1, D2>
{
    #[must_use]
    pub fn new(d1: D1, d2: D2) -> Self {
        Self { d1, d2 }
    }
}

impl<
    I,
    O1,
    O2,
    D1: Decider<ExecutionOutput = O1, Input = I>,
    D2: Decider<ExecutionOutput = O2, Input = I>,
> Decider for AndSplitDecider<I, O1, O2, D1, D2>
{
    type ExecutionOutput = (O1, O2);
    type Input = I;
    fn is_result(&mut self, input: &Self::Input, exec_output: &Self::ExecutionOutput) -> bool {
        let (o1, o2) = exec_output;
        self.d1.is_result(input, o1) && self.d2.is_result(input, o2)
    }
}

pub struct OrSplitDecider<
    I,
    O1,
    O2,
    D1: Decider<ExecutionOutput = O1, Input = I>,
    D2: Decider<ExecutionOutput = O2, Input = I>,
> {
    d1: D1,
    d2: D2,
}

impl<
    I,
    O1,
    O2,
    D1: Decider<Input = I, ExecutionOutput = O1>,
    D2: Decider<Input = I, ExecutionOutput = O2>,
> OrSplitDecider<I, O1, O2, D1, D2>
{
    #[must_use]
    pub fn new(d1: D1, d2: D2) -> Self {
        Self { d1, d2 }
    }
}

impl<
    I,
    O1,
    O2,
    D1: Decider<ExecutionOutput = O1, Input = I>,
    D2: Decider<ExecutionOutput = O2, Input = I>,
> Decider for OrSplitDecider<I, O1, O2, D1, D2>
{
    type ExecutionOutput = (O1, O2);
    type Input = I;
    fn is_result(&mut self, input: &Self::Input, exec_output: &Self::ExecutionOutput) -> bool {
        let (o1, o2) = exec_output;
        self.d1.is_result(input, o1) || self.d2.is_result(input, o2)
    }
}

use crate::Feedback;

pub struct AndSplitFeedback<
    O1,
    O2,
    F1: Feedback<ExecutionOutput = O1>,
    F2: Feedback<ExecutionOutput = O2>,
> {
    f1: F1,
    f2: F2,
}
impl<O1, O2, F1: Feedback<ExecutionOutput = O1>, F2: Feedback<ExecutionOutput = O2>>
    AndSplitFeedback<O1, O2, F1, F2>
{
    #[must_use]
    pub fn new(f1: F1, f2: F2) -> Self {
        Self { f1, f2 }
    }
}

impl<O1, O2, F1: Feedback<ExecutionOutput = O1>, F2: Feedback<ExecutionOutput = O2>> Feedback
    for AndSplitFeedback<O1, O2, F1, F2>
{
    type ExecutionOutput = (O1, O2);
    fn update(&mut self, exec_output: Self::ExecutionOutput) -> bool {
        let (o1, o2) = exec_output;
        self.f1.update(o1) && self.f2.update(o2)
    }
}

pub struct OrSplitFeedback<
    O1,
    O2,
    F1: Feedback<ExecutionOutput = O1>,
    F2: Feedback<ExecutionOutput = O2>,
> {
    f1: F1,
    f2: F2,
}
impl<O1, O2, F1: Feedback<ExecutionOutput = O1>, F2: Feedback<ExecutionOutput = O2>>
    OrSplitFeedback<O1, O2, F1, F2>
{
    #[must_use]
    pub fn new(f1: F1, f2: F2) -> Self {
        Self { f1, f2 }
    }
}

impl<O1, O2, F1: Feedback<ExecutionOutput = O1>, F2: Feedback<ExecutionOutput = O2>> Feedback
    for OrSplitFeedback<O1, O2, F1, F2>
{
    type ExecutionOutput = (O1, O2);
    fn update(&mut self, exec_output: Self::ExecutionOutput) -> bool {
        let (o1, o2) = exec_output;
        self.f1.update(o1) || self.f2.update(o2)
    }
}

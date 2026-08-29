use {crate::Feedback, std::marker::PhantomData};

pub struct AlwaysInterestingFeedback<T> {
    phantom_data: PhantomData<T>,
}
impl<T> Default for AlwaysInterestingFeedback<T> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> Feedback for AlwaysInterestingFeedback<T> {
    type ExecutionOutput = T;
    fn update(&mut self, _: Self::ExecutionOutput) -> bool {
        true
    }
}

pub struct AlwaysBoringFeedback<T> {
    phantom_data: PhantomData<T>,
}
impl<T> Default for AlwaysBoringFeedback<T> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> Feedback for AlwaysBoringFeedback<T> {
    type ExecutionOutput = T;
    fn update(&mut self, _: Self::ExecutionOutput) -> bool {
        false
    }
}

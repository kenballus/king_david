use {
    crate::Logger,
    std::{fmt::Debug, marker::PhantomData},
};

pub struct StdoutDebugLogger<T: Debug> {
    phantom_data: PhantomData<T>,
}
impl<T: Debug> Default for StdoutDebugLogger<T> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T: Debug> Logger for StdoutDebugLogger<T> {
    type Input = T;
    fn log(&mut self, input: &Self::Input, iterations: usize) {
        println!("Found result after {iterations} iterations: {input:?}");
    }
}

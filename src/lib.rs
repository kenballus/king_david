pub mod corpora;
pub mod deciders;
pub mod executors;
pub mod feedbacks;
pub mod loggers;
pub mod mutators;

// Runs the program on the given input and produces an output
pub trait Executor {
    type Input;
    type Output;
    fn run(&mut self, input: &Self::Input) -> Self::Output;
}

// Updates the feedback state (e.g., observed coverage), and returns whether this
// ExecutionOutput is interesting.
pub trait Feedback {
    type ExecutionOutput;
    fn update(&mut self, exec_output: Self::ExecutionOutput) -> bool;
}

pub trait Logger {
    type Input;
    fn log(&mut self, input: &Self::Input);
}

pub trait Mutator {
    type Input;
    fn mutate(&mut self, input: &Self::Input) -> Self::Input;
}

pub trait Decider {
    type Input;
    type ExecutionOutput;
    fn is_result(&mut self, input: &Self::Input, exec_output: &Self::ExecutionOutput) -> bool;
}

pub trait Corpus {
    type Input;
    fn select(&mut self) -> &Self::Input;
    fn add(&mut self, input: Self::Input);
}

pub struct Fuzzer<D, E, F, M, C, L> {
    decider: D,
    executor: E,
    feedback: F,
    mutator: M,
    corpus: C,
    logger: L,
}

impl<D, E, F, M, C, L> Fuzzer<D, E, F, M, C, L> {
    pub fn new(decider: D, executor: E, feedback: F, mutator: M, corpus: C, logger: L) -> Self {
        Self {
            decider,
            executor,
            feedback,
            mutator,
            corpus,
            logger,
        }
    }
}

impl<
    I,
    O,
    D: Decider<ExecutionOutput = O, Input = I>,
    E: Executor<Input = I, Output = O>,
    F: Feedback<ExecutionOutput = O>,
    M: Mutator<Input = I>,
    C: Corpus<Input = I>,
    L: Logger<Input = I>,
> Fuzzer<D, E, F, M, C, L>
{
    pub fn go(&mut self) {
        loop {
            let input = self.mutator.mutate(self.corpus.select());
            let exec_output = self.executor.run(&input);
            let is_result = self.decider.is_result(&input, &exec_output);
            let is_interesting = self.feedback.update(exec_output);
            if is_result {
                self.logger.log(&input);
            } else if is_interesting {
                self.corpus.add(input);
            }
        }
    }
}

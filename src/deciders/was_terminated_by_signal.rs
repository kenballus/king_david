use {
    crate::Decider,
    std::{collections::HashSet, process::ExitStatus},
};

#[derive(Default)]
pub struct WasTerminatedBySignal {
    found: HashSet<Vec<u8>>,
}
impl Decider for WasTerminatedBySignal {
    type ExecutionOutput = ExitStatus;
    type Input = Vec<u8>;
    fn is_result(&mut self, input: &Self::Input, exec_output: &Self::ExecutionOutput) -> bool {
        if exec_output.code().is_none() /* died due to a signal */ && !self.found.contains(input)
        /* we haven't seen this input before */
        {
            assert!(
                self.found.insert(input.clone()),
                "idk how this could happen."
            ); /* stick it into found */
            true
        } else {
            false
        }
    }
}

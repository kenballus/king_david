use {crate::Feedback, std::collections::HashSet};

#[derive(Default)]
pub struct EdgeCoverageFeedback {
    edges: HashSet<usize>,
}

impl Feedback for EdgeCoverageFeedback {
    type ExecutionOutput = HashSet<usize>;
    fn update(&mut self, mut exec_output: Self::ExecutionOutput) -> bool {
        if exec_output.is_subset(&self.edges) {
            false
        } else {
            self.edges.extend(exec_output.drain());
            true
        }
    }
}

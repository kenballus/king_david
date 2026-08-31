use crate::Feedback;

pub struct EdgeCoverageFeedback {
    map: Vec<u8>,
}

impl EdgeCoverageFeedback {
    #[must_use]
    pub fn new(map_size: usize) -> Self {
        Self {
            map: vec![0; map_size],
        }
    }
}

impl Feedback for EdgeCoverageFeedback {
    type ExecutionOutput = &'static [u8];
    fn update(&mut self, exec_output: Self::ExecutionOutput) -> bool {
        let mut interesting = false;
        for (index, _) in exec_output
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count > 0)
        {
            if self.map[index] == 0 {
                self.map[index] = 1;
                interesting = true;
            }
        }
        interesting
    }
}

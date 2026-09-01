use {
    crate::Logger,
    std::{
        fs::File,
        hash::{DefaultHasher, Hasher},
        io::Write,
        path::PathBuf,
    },
};

pub struct ResultsDirLogger {
    results_dir: PathBuf,
}
impl ResultsDirLogger {
    #[must_use]
    pub fn new(results_dir: PathBuf) -> Self {
        assert!(results_dir.exists(), "Results dir doesn't exist");
        assert!(results_dir.is_dir(), "Results dir is not a directory");
        Self { results_dir }
    }
}

impl Logger for ResultsDirLogger {
    type Input = Vec<u8>;
    fn log(&mut self, input: &Self::Input, _: usize) {
        let mut hasher = DefaultHasher::new();
        hasher.write(input);
        let result_filename = self.results_dir.join(hasher.finish().to_string());
        if result_filename.exists() {
            eprintln!(
                "Attempt to save result {} failed. File already exists. Leftover from previous run?",
                result_filename.display()
            );
        } else {
            let mut result_file = File::create(result_filename)
                .expect("Failed to open result file. Permission problem?");
            result_file
                .write_all(input.as_slice())
                .expect("Failed to write to result file. Permission problem?");
        }
    }
}

use {
    crate::Executor,
    std::{
        ffi::OsString,
        io::Write,
        process::{Command, ExitStatus, Stdio},
    },
};

pub struct BlackBoxStdinExecutor {
    path: OsString,
    argv: Vec<OsString>,
    // I would put an envp in here, but Rust doesn't have a safe way of making envvars that don't
    // contain '=' (I think).
}
impl BlackBoxStdinExecutor {
    #[must_use]
    pub fn new(path: OsString, argv: Vec<OsString>) -> Self {
        Self { path, argv }
    }
}

impl Executor for BlackBoxStdinExecutor {
    type Output = ExitStatus;
    type Input = Vec<u8>;
    fn run(&mut self, bytes_for_stdin: &Self::Input) -> Self::Output {
        let mut child = Command::new(&self.path)
            .args(self.argv.iter())
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to run the fuzzed process! Invalid path?");
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(bytes_for_stdin)
                .expect("Failed to write to the fuzzed process! Is it reading?");
        } else {
            panic!("failed to acquire child's stdin. idk why this would happen");
        }
        child
            .wait()
            .expect("Failed to wait on the fuzzed process! idk why this would happen")
    }
}

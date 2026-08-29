use {
    crate::Executor,
    libc::{
        F_GETFD, F_SETFD, FD_CLOEXEC, IPC_CREAT, IPC_EXCL, IPC_PRIVATE, close, dup2, fcntl, shmat,
        shmget,
    },
    nix::unistd::pipe,
    std::{
        collections::HashSet,
        ffi::OsString,
        fs::{File, OpenOptions},
        io::{Read, Seek, SeekFrom, Write},
        os::{
            fd::AsRawFd,
            unix::process::{CommandExt, ExitStatusExt},
        },
        process::{Command, ExitStatus, Stdio},
        ptr, slice,
    },
};

fn read_pipe_word(mut pipe: &File) -> [u8; 4] {
    let mut result = [0; 4];
    pipe.read_exact(&mut result).unwrap();
    result
}

const AFL_HANDSHAKE_BYTES: [u8; 4] = *b"\x01LFA";

pub struct AflppStdinExecutor<'a> {
    stdin_file: File,
    coverage_shmem: &'a mut [u8],
    read_pipe: File,
    write_pipe: File,
}
impl AflppStdinExecutor<'_> {
    #[must_use]
    pub fn new(path: &OsString, argv: &[OsString], stdin_filename: &str) -> Self {
        // this syscall sequence was ripped directly from running strace on afl-showmap
        // strace -ff afl-showmap -o /dev/null -- ./example_targets/manual_strcmp_afl/main
        const AFL_MAP_SIZE: usize = 8_388_608;

        let stdin_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(stdin_filename)
            .unwrap();

        let shmem_id = unsafe { shmget(IPC_PRIVATE, AFL_MAP_SIZE, IPC_CREAT | IPC_EXCL | 0o600) };
        let shmem_addr = unsafe { shmat(shmem_id, ptr::null(), 0) };

        let (read_pipe, child_write_pipe) = pipe().unwrap();
        let (child_read_pipe, write_pipe) = pipe().unwrap();

        let mut forkserver = Command::new(path);
        forkserver
            .args(argv.iter())
            .env("__AFL_SHM_ID", shmem_id.as_raw_fd().to_string())
            .env("AFL_MAP_SIZE", AFL_MAP_SIZE.to_string())
            .env("LD_BIND_NOW", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        // This is outside the move closure so it isn't closed in the parent.
        let stdin_file_fd = stdin_file.as_raw_fd();
        unsafe {
            forkserver.pre_exec(move || {
                // dup the pipes into 198 and 199, which is where AFL++ expects them.
                dup2(child_read_pipe.as_raw_fd(), 198);
                dup2(child_write_pipe.as_raw_fd(), 199);
                // The move on the closure will cause chlid_read_pipe and child_write_pipe to be
                // closed in both the parent and the child, which is what we want.

                // dupe the input file into stdin
                dup2(stdin_file_fd, 0);
                close(stdin_file_fd);

                // mark the new stdin so we don't lose it when we exec
                fcntl(0, F_SETFD, fcntl(0, F_GETFD) & !FD_CLOEXEC);

                Ok(())
            })
        };
        #[allow(clippy::zombie_processes)]
        forkserver.spawn().expect("failed to start forkserver!");

        let read_pipe = File::from(read_pipe);

        assert!(
            read_pipe_word(&read_pipe) == AFL_HANDSHAKE_BYTES,
            "AFL++ handshake initiator was weird"
        );

        let mut write_pipe = File::from(write_pipe);
        write_pipe
            .write_all(&AFL_HANDSHAKE_BYTES.map(|x| 0xff - x))
            .unwrap();

        let _flags = read_pipe_word(&read_pipe); // TODO: figure out what to do with this.

        let map_size = read_pipe_word(&read_pipe);

        assert!(
            read_pipe_word(&read_pipe) == AFL_HANDSHAKE_BYTES,
            "AFL++ handshake initiator was weird"
        );

        Self {
            coverage_shmem: unsafe {
                slice::from_raw_parts_mut(
                    shmem_addr.cast(),
                    u32::from_ne_bytes(map_size).try_into().unwrap(),
                )
            },
            read_pipe,
            write_pipe,
            stdin_file,
        }
    }
}

impl Executor for AflppStdinExecutor<'_> {
    type Output = (ExitStatus, HashSet<usize>);
    type Input = Vec<u8>;
    fn run(&mut self, bytes_for_stdin: &Self::Input) -> Self::Output {
        self.coverage_shmem.iter_mut().for_each(|x| *x = 0);

        // write the bytes to stdin
        self.stdin_file.seek(SeekFrom::Start(0)).unwrap();
        self.stdin_file.write_all(bytes_for_stdin).unwrap();
        self.stdin_file
            .set_len(bytes_for_stdin.len().try_into().unwrap())
            .unwrap();
        self.stdin_file.seek(SeekFrom::Start(0)).unwrap();

        self.write_pipe.write_all(b"\x00\x00\x00\x00").unwrap(); // request to fork
        let _some_kind_of_counter_increasing_by_twos = read_pipe_word(&self.read_pipe); // not sure what this is
        let exit_status = ExitStatus::from_raw(i32::from_ne_bytes(read_pipe_word(&self.read_pipe)));

        let coverage = self
            .coverage_shmem
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count > 0)
            .map(|(index, _)| index)
            .collect();

        (exit_status, coverage)
    }
}

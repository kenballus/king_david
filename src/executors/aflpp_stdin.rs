use {
    crate::Executor,
    libc::{
        F_GETFD, F_SETFD, FD_CLOEXEC, IPC_CREAT, IPC_EXCL, IPC_PRIVATE, IPC_RMID, close, dup2,
        fcntl, shmat, shmctl, shmget,
    },
    nix::unistd::pipe,
    std::{
        ffi::OsString,
        fs::{File, OpenOptions},
        io::{Read, Seek, SeekFrom, Write},
        os::{
            fd::AsRawFd,
            unix::process::{CommandExt, ExitStatusExt},
        },
        process::{Child, Command, ExitStatus, Stdio},
        ptr, slice,
    },
};

// Represents an AFL shmem. Note that only a prefix of this shmem will be used for the coverage map.
// This object represents an entire shmem, so you should be slicing the result of .as_slice() if you
// want just the coverage map, and not the crap that comes after.
const AFL_MAP_SIZE: usize = 8_388_608;
struct AflSysVShmem {
    id: i32,
    addr: *mut u8,
    size: usize,
}

impl AflSysVShmem {
    fn as_slice(&self) -> &'static [u8] {
        unsafe { slice::from_raw_parts(self.addr.cast(), self.size) }
    }

    fn as_mut_slice(&mut self) -> &'static mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.addr.cast(), self.size) }
    }
}

// There is no Drop impl for AflSysVShmem because that would mean the lifetime shouldn't be static.
// I want it to be that any time you make one of these, it lives until the end of the process. This
// makes it super easy to reason about lifetimes. I really tried to make this work, but it required
// cluttering my traits enough that I think this is the best option.

impl Default for AflSysVShmem {
    fn default() -> Self {
        // Create a shmem
        let id = unsafe { shmget(IPC_PRIVATE, AFL_MAP_SIZE, IPC_CREAT | IPC_EXCL | 0o600) };
        assert!(id != -1);

        // Map it into memory
        let addr = unsafe { shmat(id, ptr::null(), 0) }.cast();
        assert!(addr != usize::MAX as *mut u8);

        // Mark it to be destroyed when it's no longer mapped anywhere.
        // Note that this has to happen after mapping the shmem, or else it will be immediately
        // destroyed.
        assert!(unsafe { shmctl(id, IPC_RMID, ptr::null_mut()) } == 0);

        Self {
            id,
            addr,
            size: AFL_MAP_SIZE,
        }
    }
}

fn read_pipe_word(mut pipe: &File) -> [u8; 4] {
    let mut result = [0; 4];
    pipe.read_exact(&mut result).unwrap();
    result
}

pub struct AflppStdinExecutor {
    stdin_file: File,
    shmem: AflSysVShmem,
    pub map_size: usize, // the size of the used portion of the shmem
    read_pipe: File,
    write_pipe: File,
    forkserver: Child,
}
impl AflppStdinExecutor {
    #[must_use]
    pub fn new(path: &OsString, argv: &[OsString], stdin_filename: &str) -> Self {
        // this syscall sequence was ripped directly from running strace on afl-showmap
        // strace -ff afl-showmap -o /dev/null -- ./example_targets/manual_strcmp_afl/main
        const AFL_HANDSHAKE_BYTES: [u8; 4] = *b"\x01LFA";

        let stdin_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(stdin_filename)
            .unwrap();

        // These pipes are how we send and receive data from the forkserver.
        let (read_pipe, child_write_pipe) = pipe().unwrap();
        let (child_read_pipe, write_pipe) = pipe().unwrap();

        let shmem = AflSysVShmem::default();

        let mut forkserver = Command::new(path);
        forkserver
            .args(argv.iter())
            .env("__AFL_SHM_ID", shmem.id.to_string())
            .env("AFL_MAP_SIZE", shmem.size.to_string())
            .env("LD_BIND_NOW", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        // This is outside the move closure so it isn't closed in the parent.
        // We will be repeatedly seeking/writing to this fd, so we need to make sure it remains
        // open.
        let stdin_file_fd = stdin_file.as_raw_fd();
        unsafe {
            forkserver.pre_exec(move || {
                // dup the pipes into 198 and 199, which is where AFL++ expects them.
                assert!(dup2(child_read_pipe.as_raw_fd(), 198) != -1);
                assert!(dup2(child_write_pipe.as_raw_fd(), 199) != -1);
                // The move on the closure will cause child_read_pipe and child_write_pipe to be
                // closed in both the parent and the child, which is what we want.
                // (The duped 198 and 199 remain open in the child)

                // dupe the input file into stdin
                assert!(dup2(stdin_file_fd, 0) != -1);
                close(stdin_file_fd);

                // mark the new stdin so we don't lose it when we exec
                let flags = fcntl(0, F_GETFD);
                assert!(flags != -1);
                assert!(fcntl(0, F_SETFD, flags & !FD_CLOEXEC) != -1);

                Ok(())
            })
        };
        let forkserver = forkserver.spawn().expect("failed to start forkserver!");

        let read_pipe = File::from(read_pipe);

        assert!(
            read_pipe_word(&read_pipe) == AFL_HANDSHAKE_BYTES,
            "AFL++ handshake initiator was weird"
        );

        let mut write_pipe = File::from(write_pipe);
        write_pipe
            .write_all(&AFL_HANDSHAKE_BYTES.map(|x| 0xff - x))
            .unwrap();

        assert!(
            u32::from_ne_bytes(read_pipe_word(&read_pipe)) == 1,
            "unexpected flags. build with the latest afl++"
        );

        let map_size: usize = u32::from_ne_bytes(read_pipe_word(&read_pipe))
            .try_into()
            .unwrap();
        assert!(map_size < shmem.size);

        assert!(
            read_pipe_word(&read_pipe) == AFL_HANDSHAKE_BYTES,
            "AFL++ handshake terminator was weird"
        );

        Self {
            stdin_file,
            shmem,
            map_size,
            read_pipe,
            write_pipe,
            forkserver,
        }
    }

    fn get_shmem_mut_slice(&mut self) -> &'static mut [u8] {
        &mut self.shmem.as_mut_slice()[..self.map_size]
    }

    fn get_shmem_slice(&self) -> &'static [u8] {
        &self.shmem.as_slice()[..self.map_size]
    }
}

impl Executor for AflppStdinExecutor {
    type Output = (ExitStatus, &'static [u8]);
    type Input = Vec<u8>;
    fn run(&mut self, bytes_for_stdin: &Self::Input) -> Self::Output {
        self.get_shmem_mut_slice().fill(0);

        // write the bytes to stdin
        self.stdin_file.seek(SeekFrom::Start(0)).unwrap();
        self.stdin_file.write_all(bytes_for_stdin).unwrap();
        self.stdin_file
            .set_len(bytes_for_stdin.len().try_into().unwrap())
            .unwrap();
        self.stdin_file.seek(SeekFrom::Start(0)).unwrap();

        self.write_pipe.write_all(b"\x00\x00\x00\x00").unwrap(); // request to fork
        let _pid = read_pipe_word(&self.read_pipe);
        let exit_status = ExitStatus::from_raw(i32::from_ne_bytes(read_pipe_word(&self.read_pipe)));

        (exit_status, self.get_shmem_slice())
    }
}

impl Drop for AflppStdinExecutor {
    fn drop(&mut self) {
        // Avoid creating a zombie process.
        self.forkserver.wait().unwrap();
    }
}

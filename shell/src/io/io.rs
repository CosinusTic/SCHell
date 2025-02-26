use libc::{dup, dup2};
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};

pub fn capture_fun_stdout<F>(func: F) -> String
where
    F: FnOnce(),
{
    // Create a pipe
    let (mut reader, writer) = os_pipe::pipe().expect("Failed to create pipe");

    // Save original stdout
    let original_stdout = io::stdout().as_raw_fd();
    let saved_stdout = unsafe { File::from_raw_fd(dup(original_stdout)) };

    // Redirect stdout to the writer of the pipe
    unsafe {
        dup2(writer.as_raw_fd(), original_stdout);
    }

    // Run the custom function
    func();

    // Flush to ensure all output is written to the pipe
    io::stdout().flush().expect("Failed to flush stdout");
    drop(writer);

    // Restore original stdout
    unsafe {
        dup2(saved_stdout.as_raw_fd(), original_stdout);
    }

    // Read the output from the reader
    let mut output = String::new();
    reader
        .read_to_string(&mut output)
        .expect("Failed to read from pipe");

    output
}
/* Usage example:
* let name = "Nathan";
let out_arg = capture_stdout(|| {
    foo_arg(name);
});*/

pub fn capture_proc_stdout() -> String {
    // Creating the pipe
    let (mut reader, writer) = os_pipe::pipe().expect("Failed to pipe process");

    // Save original stdout to file descriptor
    let orig_fd = io::stdout().as_raw_fd();
    let saved_fd = unsafe { File::from_raw_fd(dup(orig_fd)) };

    // Redirect stdout to writer
    unsafe { dup2(writer.as_raw_fd(), orig_fd) };

    drop(writer);

    let mut out = String::new();
    reader
        .read_to_string(&mut out)
        .expect("Failed to read from pipe");

    // Restore original stdout
    unsafe { dup2(saved_fd.as_raw_fd(), orig_fd) };

    out
}

use libc::{dup, dup2};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};

pub fn capt_stdout<F>(func: F) -> String
where
    F: FnOnce(),
{
    let (mut reader, writer) = os_pipe::pipe().expect("Failed to create pipe");

    let original_stdout = io::stdout().as_raw_fd();
    let saved_stdout = unsafe { File::from_raw_fd(dup(original_stdout)) };
    unsafe {
        dup2(writer.as_raw_fd(), original_stdout);
    }

    func();

    io::stdout().flush().expect("Failed to flush stdout");
    drop(writer);
    unsafe {
        dup2(saved_stdout.as_raw_fd(), original_stdout);
    }

    let mut output = String::new();
    reader
        .read_to_string(&mut output)
        .expect("Failed to read from pipe");

    output.split("\\n").collect()
}

pub fn load_f_lines(path: &str) -> Vec<String> {
    let f: File = File::open(path).expect("Failed to open file");
    let br = BufReader::new(f);
    let r = br.lines().map(|l| l.unwrap());
    r.collect()
}

/*
pub fn load_f_first_line(path: &str) -> &String {
    let f: File = File::open(path).expect("Failed to open file");
    let br = BufReader::new(f);
    let r = br.lines().map(|l| l.unwrap());
    let l: Vec<String> = r.collect();
    l.first().unwrap()
}
*/

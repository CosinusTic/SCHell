use crate::io::load_f_lines;
use std::fs::{read_dir, File};
use std::io::{stderr, stdin, stdout, BufRead, BufReader};
use std::ops::Add;

fn curr_pid() -> u32 {
    match sysinfo::get_current_pid() {
        Ok(pid) => pid.as_u32(),
        Err(_) => 0,
    }
}

fn pid_fpath(pid: &u32) -> String {
    String::from("/proc/") + pid.to_string().as_str()
}

pub fn my_pid() {
    match curr_pid() {
        0 => println!("Failed to get current PID"),
        _ => println!("Current PID is {}", curr_pid()),
    }
}

pub fn l_procfiles() {
    let pid = curr_pid();
    match pid {
        0 => println!("Failed to retrieve current PID"),
        _ => {
            let path: String = pid_fpath(&pid);
            let rddir = read_dir(path).unwrap();
            for f in rddir {
                println!("{:?}", f.as_ref().unwrap().path());
            }
        }
    }
}

pub fn get_RAM_size() {
    let start_path: &str = &"/sys/firmware/memmap/0/start";
    let end_path: &str = &"/sys/firmware/memmap/0/end";
    let type_path: &str = &"/sys/firmware/memmap/0/type";

    let start_lines = load_f_lines(start_path);
    let end_lines = load_f_lines(end_path);
    let type_lines = load_f_lines(type_path);
    let t = type_lines.first().unwrap();
    let e = end_lines.first().unwrap();
    let s = start_lines.first().unwrap();
    println!("Type: {}", t);
    println!("Start: {}", s);
    println!("End: {}", e);
}

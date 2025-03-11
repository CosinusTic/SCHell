use crate::io::load_f_lines;
use ascii_to_hex;
use std::fs::{read_dir, File};
use std::i64;
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

pub fn my_pid(_args: Vec<String>) {
    match curr_pid() {
        0 => println!("Failed to get current PID"),
        _ => println!("Current PID is {}", curr_pid()),
    }
}

pub fn l_procfiles(_args: Vec<String>) {
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

pub fn get_RAM_size(_args: Vec<String>) {
    let start_path: &str = &"/sys/firmware/memmap/0/start";
    let end_path: &str = &"/sys/firmware/memmap/0/end";
    let type_path: &str = &"/sys/firmware/memmap/0/type";

    let start_lines = load_f_lines(start_path);
    let end_lines = load_f_lines(end_path);
    let type_lines = load_f_lines(type_path);
    let t = type_lines.first().unwrap();
    let e = end_lines.first().unwrap();
    let s = start_lines.first().unwrap();
    let s_trimmed = s.trim_start_matches("0x");
    let e_trimmed = e.trim_start_matches("0x");
    let s_val: i64 = i64::from_str_radix(s_trimmed, 16).unwrap(); // convert hex to int
    let e_val: i64 = i64::from_str_radix(e_trimmed, 16).unwrap();
    println!("Type: {}", t);
    println!("Start: {}", s);
    println!("End: {}", e);
    println!("Start (as int): {}", s_val);
    println!("End (as int): {}", e_val);
    println!("RAM size: {} kBytes", (e_val - s_val) / (8 * 1024));
}

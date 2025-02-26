use std::fs::{read_dir, File};
use std::io::{stderr, stdin, stdout};

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

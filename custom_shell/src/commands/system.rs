use sysinfo::get_current_pid;

pub fn current_pid() -> i32 {
    match sysinfo::get_current_pid() {
        Ok(pid) => pid.as_u32() as i32,
        Err(_) => -1
    }
}

pub fn my_pid() {
    match current_pid() {
        (-1) => println!("Failed to get current PID"),
        _ => println!("Current PID is {}", current_pid()),
    }
}

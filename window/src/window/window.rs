use std::process::Command;

pub fn spawn_terminal_window() {
    let bin: &str = "/home/nathan/Documents/Personal/Projects/my_shell/shell/target/debug/myshell";
    Command::new("gnome-terminal")
        .arg("--")
        .arg(bin)
        .spawn()
        .expect("Failed to spawn terminal instance");
}

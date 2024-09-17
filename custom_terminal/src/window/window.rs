use std::process::Command;

pub fn spawn_terminal_window() {
    Command::new("gnome-terminal")
        .arg("--")
        .arg("/home/nathan/Documents/Personal/Projects/my_terminal/custom_shell/target/debug/custom_shell")
        .spawn()
        .expect("Failed to spawn terminal instance");
}


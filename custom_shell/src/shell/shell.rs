use std::process::Command;

pub fn parse_command(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}


pub fn execute_command(command: Vec<&str>) {
    if command.is_empty() {
        return;
    }

    let mut cmd = Command::new(command[0]);

    if command.len() > 1 {
        cmd.args(&command[1..]);
    }

    match cmd.spawn() {
        Ok(mut child) => {
            child.wait().expect("Command wasn't running");
        }
        Err(e) => eprintln!("Error executing command {}", e),
    }
}


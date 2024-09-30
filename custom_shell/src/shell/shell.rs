use std::collections::HashMap;
use crate::commands::basic::*;
use crate::commands::system::*;
use std::any::Any;

type Command = fn();

pub fn parse_command(input: &str) -> Vec<String> {
    input.split_whitespace().map(String::from).collect()
}

pub fn register_commands() -> HashMap<String, Command> {
    let mut commands = HashMap::new();
    commands.insert("hello".to_string(), hello_command as Command);
    commands.insert("mypid".to_string(), my_pid as Command);

    commands
}

pub fn execute_command(command: Vec<String>, registered_commands: &HashMap<String, Command>) -> bool {
    if command.is_empty() {
        return false;
    }
    
    if let Some(cmd) = registered_commands.get(&command[0]) {
        cmd();
        true
    }
    else {
        false
    }
}


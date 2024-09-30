use std::io::{self, Write};

use custom_shell::shell::shell::*;

fn main() {
    let commands = register_commands();
    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break;
        }
        
        let command = parse_command(input);
        execute_command(command, &commands);
    }
}   

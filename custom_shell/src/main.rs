use std::io::{self, Write};

mod shell;

fn main() {
    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let command = shell::shell::parse_command(input);
        shell::shell::execute_command(command);
    }
}   

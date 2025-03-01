use myshell::ast::AstNode;
use myshell::io::*;
use myshell::parsing::parse_cmd;
use myshell::shell::shell::*;
use myshell::utils::str_to_vec;
use std::io::{stdin, stdout, Write};

fn main() {
    /* ---------------- Basic shell ----------------- */
    /*

    let commands = register_commands();
    let commands_str = register_commands_str();
    loop {
        print!("> ");

        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }

        let command = parse_command(input);
        execute_command(command, &commands, &commands_str);
    }

    */
    /* -------------------------- Stdout tests ------------------- */
    /*
    let out = capture_fun_stdout(|| {
        foo();
        bar();
    });

    println!("Captured {}", out);
    foo();
    */

    /* --------------- AST shell test ---------------------- */
    let commands = register_commands();
    let commands_str = register_commands_str();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read command.");
        println!("Trimmed input: {:?}", str_to_vec(input.as_str()));
        // -------------- Commands --------------
        let cmd1 = &[
            "ls".to_string(),
            ".".to_string(),
            "|".to_string(),
            "hello".to_string(),
        ];

        let cmd2 = &[
            "ls".to_string(),
            "-la".to_string(),
            ".".to_string(),
            "|".to_string(),
            "grep".to_string(),
            "zizi".to_string(),
        ];

        let cmd3 = &[
            "ls".to_string(),
            ".".to_string(),
            "|".to_string(),
            "hello".to_string(),
        ];

        let cmd4 = &["ls".to_string(), ".".to_string()];
        let cmd5 = &[
            "ls".to_string(),
            ".".to_string(),
            "|".to_string(),
            "grep".to_string(),
            "zizi".to_string(),
        ];
        let node = parse_cmd(cmd5);
        // node.debug();
        exec(node, &commands, &commands_str);
    }
}

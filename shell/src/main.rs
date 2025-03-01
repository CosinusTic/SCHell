use myshell::ast::AstNode;
use myshell::io::*;
use myshell::parsing::parse_cmd;
use myshell::shell::shell::*;
use myshell::utils::str_to_vec;
use std::io::{stdin, stdout, Write};

fn foo() {
    println!("This is some nice output");
}

fn bar() {
    println!("This is also some nice output");
}

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
        let mut cmd: Vec<&str> = Vec::new();
        let _ = input.split(" ").map(|x| {
            cmd.push(x);
            println!("x: {}", x);
            x
        });
        println!("cmd: {:?}", cmd);
        let node_test: AstNode = parse_cmd(&[
            "ls".to_string(),
            ".".to_string(),
            "|".to_string(),
            "hello".to_string(),
        ]);
        node_test.debug();
        // let node: AstNode = parse_cmd(input.split("").collect());
    }
}

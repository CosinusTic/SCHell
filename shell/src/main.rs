use myshell::ast::AstNode;
use myshell::io::*;
use myshell::shell::shell::*;
use std::io::{stdin, stdout, Write};
use std::process::Stdio;

fn foo() {
    println!("This is some nice output");
}

fn bar() {
    println!("This is also some nice output");
}

fn main() {
    /*
    let n = String::from("ls");
    let a = Vec::new();
    let ast: AstNode = AstNode::Command { name: n, args: a };
    ast.debug(0);

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

    let out = capture_fun_stdout(|| {
        foo();
        bar();
    });

    println!("Captured {}", out);
    foo();
}

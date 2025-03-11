use crate::io::capt_stdout;
use crate::utils::trim_eol;
use crate::{ast::AstNode, commands::*};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

type Command = fn(Vec<String>);

pub fn parse_command(input: &str) -> Vec<String> {
    input.split_whitespace().map(String::from).collect()
}

pub fn register_commands() -> HashMap<String, Command> {
    let mut commands = HashMap::new();
    commands.insert(String::from("hello"), greet as Command);
    commands.insert(String::from("mypid"), my_pid as Command);
    commands.insert(String::from("lprocf"), l_procfiles as Command);
    commands.insert(String::from("ls"), ls as Command);
    commands.insert(String::from("echo"), echo as Command);
    commands.insert(String::from("grep"), grep as Command);
    commands
}

pub fn execute_command(
    command: Vec<String>,
    registered_commands: &HashMap<String, Command>,
) -> bool {
    if command.is_empty() {
        return false;
    }

    if let Some(cmd) = registered_commands.get(&command[0]) {
        cmd(command[1..].to_vec());
        true
    } else {
        false
    }
}

pub fn exec(node: AstNode, registered_commands: &HashMap<String, Command>) {
    match node {
        AstNode::Command { name, args } => {
            if let Some(cmd) = registered_commands.get(&name) {
                cmd(args.clone());
                println!(
                    "[FINAL] Executing command: \"{}\" (with args \"{:?}\")",
                    name, args
                )
            } else {
                println!("[FINAL] Cannot find command: \"{}\"", &name);
                return;
            }
        }
        AstNode::Pipe { left, right } => {
            let out = capt_stdout(|| {
                exec(*left, registered_commands);
            });
            if let AstNode::Command { name, args } = *right {
                let mut v: Vec<String> = vec![out];
                v.extend(args.iter().map(|s| s.trim_end().to_string()));
                if let Some(cmd) = registered_commands.get(&name) {
                    cmd(v);
                    return;
                }
            } else {
                exec(*right, registered_commands);
            }
        }
        AstNode::Redirect {
            command,
            file,
            append,
        } => {
            println!("Handling redirect");
        }
        AstNode::Sequence { left, right } => {
            println!("Handling seq");
        }
    }
    /* Outer if: big match on node;
    * ex match node {
    * AstNode::Command {name, args} => {
    *  inner ifs go here
    * } ...
    *
    * Within each match statement, check for registered commands or
    * registered command_str:
    *  if let Some(cmd) = registered_commands.get(&name) {
                cmd(); // Call the registered command without arguments
                return;
            }

            // 2. Check for registered commands with arguments
            if let Some(cmd_str) = registered_commands_str.get(&name) {
                if args.len() == 1 {
                    cmd_str(&args[0]); // Pass the single argument
                } else {
                    println!("Error: {} requires exactly one argument", name);
                }
                return;
            }
    */

    /*
    * Command for pipe works differently:
    * once captured pipe, you have 2 commands, you must execute the left (with the match above)
    * and capture its outpout
    * Then choose: if it is a command with a single argument, just execute it with parsed
    * command and its argument, if it is a command with two arguments then either:
        - potentially use the output as second argument (and create new hashmap for 2 arguments fonctions)
        - Use the stdin (must register first command's output to stdout and redirect it to stdin).
        then in the function codes (ex my_grep() read from stdin using io::stdin().lock() -> (Vec<String>))
    */
}

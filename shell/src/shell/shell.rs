use crate::{ast::AstNode, commands::*};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

type Command = fn();
type CommandStr = fn(&str);

pub fn parse_command(input: &str) -> Vec<String> {
    input.split_whitespace().map(String::from).collect()
}

pub fn register_commands() -> HashMap<String, Command> {
    let mut commands = HashMap::new();
    commands.insert("hello".to_string(), greet as Command);
    commands.insert("mypid".to_string(), my_pid as Command);
    commands.insert("lprocf".to_string(), l_procfiles as Command);
    commands
}

pub fn register_commands_str() -> HashMap<String, CommandStr> {
    let mut commands = HashMap::new();
    commands.insert("ls".to_string(), ls as CommandStr);
    commands.insert("echo".to_string(), echo as CommandStr);
    commands
}

pub fn execute_command(
    command: Vec<String>,
    registered_commands: &HashMap<String, Command>,
    registered_commands_str: &HashMap<String, CommandStr>,
) -> bool {
    if command.is_empty() {
        return false;
    }

    if let Some(cmd) = registered_commands.get(&command[0]) {
        cmd();
        true
    } else if let Some(cmd) = registered_commands_str.get(&command[0]) {
        cmd(&command[1]);
        true
    } else {
        false
    }
}

pub fn exec(
    node: AstNode,
    registered_commands: &HashMap<String, Command>,
    registered_commands_str: &HashMap<String, CommandStr>,
) {
    println!("Node at execution stage: {:?}", node);
    match node {
        AstNode::Command { name, args } => {
            if let Some(cmd) = registered_commands.get(&name) {
                // cmd();
                println!("Executing command: \"{}\"", name);
            } else if let Some(cmd) = registered_commands_str.get(&name) {
                // cmd(&args[0]);
                println!("Executing command: \"{}\" (with args \"{:?}\")", name, args)
            } else {
                println!("Cannot find command: \"{}\"", &name);
                return;
            }
        }
        AstNode::Pipe { left, right } => {
            /*exec(
                AstNode::Command {
                    name: left.into(),
                    args: &[],
                },
                registered_commands,
                registered_commands_str,
            );*/
            println!("Encountered pipe (Left: {:?}, right: {:?})", left, right);
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
    /* Outer if: big match on node
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

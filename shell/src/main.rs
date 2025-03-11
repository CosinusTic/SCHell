use shell::commands::get_RAM_size;
use shell::io::*;
use shell::parsing::parse;
use shell::shell::shell::*;
use shell::utils::str_to_vec;
use shell::{ast::AstNode, commands::grep};
use std::io::{stdin, stdout, BufRead, Result, Write};

fn main() -> Result<()> {
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
    let out = capt_stdout(|| {
        myshell::commands::greet();
    });
    */

    /* --------------- AST shell test ---------------------- */
    let commands = register_commands();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        /*let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read command.");
        println!("Trimmed input: {:?}", str_to_vec(input.as_str()));
        let v = str_to_vec(input.as_str());*/
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let mut lines: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
        let slice: &[String] = &lines;

        let cmd_slice: &[String] = &lines;
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
        let node = parse(cmd_slice);
        // node.debug();
        exec(node, &commands);

        // get_RAM_size();
    }
}

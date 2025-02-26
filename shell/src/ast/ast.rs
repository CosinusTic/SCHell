#[derive(Debug)]
pub enum AstNode {
    Command {
        // A simple command
        name: String,
        args: Vec<String>,
    },
    Pipe {
        // A pipeline command: output of left is passed as input for right
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Redirect {
        // ">" the output of the command is written (or appended) into the file
        command: Box<AstNode>,
        file: String,
        append: bool,
    },
    Sequence {
        // Just two commands executed (left then right)
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}

impl AstNode {
    pub fn debug(&self, level: usize) {
        let indent = "  ".repeat(level);

        match self {
            AstNode::Command { name, args } => {
                println!("{}Command:", indent);
                println!("{}  Name: {}", indent, name);
                if !args.is_empty() {
                    println!("{}  Args: {:?}", indent, args);
                }
            }
            AstNode::Pipe { left, right } => {
                println!("{}Pipe:", indent);
                println!("{}  Left:", indent);
                left.debug(level + 2);
                println!("{}  Right:", indent);
                right.debug(level + 2);
            }
            AstNode::Redirect {
                command,
                file,
                append,
            } => {
                println!("{}Redirect:", indent);
                println!("{}  File: {}", indent, file);
                println!("{}  Append: {}", indent, append);
                println!("{}  Command:", indent);
                command.debug(level + 2);
            }
            AstNode::Sequence { left, right } => {
                println!("{}Sequence:", indent);
                println!("{}  Left:", indent);
                left.debug(level + 2);
                println!("{}  Right:", indent);
                right.debug(level + 2);
            }
        }
    }
}

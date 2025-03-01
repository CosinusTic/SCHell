use crate::ast::AstNode;

// May need for a tokenisation function
// With input.trim ?

// Create ast nodes recursively depending on command depending on tokens
// Tokens may be of the form ["ls", "-l", "|", "echo", "hello"]
pub fn parse_command(tokens: &[String]) -> AstNode {
    // Recursive parsing to get to the command in the end
    if tokens.contains(&"|".to_string()) {
        let pos = tokens.iter().position(|x| x == "|").unwrap();
        let (left, right) = tokens.split_at(pos);
        return AstNode::Pipe {
            left: Box::new(parse_command(left)),
            right: Box::new(parse_command(&right[1..])),
        };
    }
    if tokens.contains(&">".to_string()) {
        let pos = tokens.iter().position(|x| x == ">").unwrap();
        let (command, file) = tokens.split_at(pos);
        return AstNode::Redirect {
            command: Box::new(parse_command(command)),
            file: file.get(0).unwrap().to_string(),
            append: false,
        };
    }
    if tokens.contains(&"&&".to_string()) {
        let pos = tokens.iter().position(|x| x == "&&").unwrap();
        let (left, right) = tokens.split_at(pos);
        return AstNode::Sequence {
            left: Box::new(parse_command(left)),
            right: Box::new(parse_command(&right[1..])),
        };
    }

    // Otherwise return a simple command
    AstNode::Command {
        name: tokens[0].clone(),
        args: tokens[1..].to_vec(),
    }
}

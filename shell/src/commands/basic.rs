use crate::utils::trim_eol;
use std::fs;

pub fn greet(_args: Vec<String>) {
    println!("Hello, World!");
}

pub fn echo(args: Vec<String>) {
    println!("{}", args.join(" "));
}

pub fn ls(args: Vec<String>) {
    let files = fs::read_dir(&args[0]).unwrap();

    for file in files {
        println!("{:?}", file.as_ref().unwrap().path());
    }
}

pub fn grep(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("Usage: grep <pattern> <text>");
    } else {
        let pattern = &args[0];
        let input = &args[1..];
        // println!("---------------------");
        /* println!(
                    "pattern ({} elements): {:?}, input: {:?}",
                    trim_eol(pattern).len(),
                    trim_eol(pattern),
                    input
                );
        */
        let p = trim_eol(pattern);
        // println!("---------------------");
        for line in p {
            // println!("Line: {}", line);
            if line.to_lowercase().contains(&input[0]) {
                println!("[GREP] Match found: {}", line);
            }
        }
    }
}

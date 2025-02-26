use std::fs;

pub fn hello_command() {
    println!("Hello World");
}

pub fn ls(path: &str) {
    let files = fs::read_dir(path).unwrap();

    for file in files {
        println!("{:?}", file.as_ref().unwrap().path());
    }
}

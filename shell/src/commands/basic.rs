use std::fs;

pub fn greet() {
    println!("Hello World");
}

pub fn echo(out: &str) {
    println!("{}", out);
}

pub fn ls(path: &str) {
    let files = fs::read_dir(path).unwrap();

    for file in files {
        println!("{:?}", file.as_ref().unwrap().path());
    }
}

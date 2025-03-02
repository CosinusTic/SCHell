#[cfg(test)]
use super::*;
use crate::commands::*;
use crate::io::capt_stdout;

#[test]
pub fn test1() {
    assert_eq!("1", "1");
}

#[test]
pub fn test2() {
    assert_ne!("1", "2");
}

#[test]
pub fn test_greet1() {
    let expected = "Hello, World!";
    let actual = capt_stdout(|| {
        println!("Hello, World!");
    });
    println!("Actual: {}", actual);

    assert_eq!(expected, actual);
}

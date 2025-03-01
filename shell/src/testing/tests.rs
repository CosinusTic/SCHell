#[cfg(test)]
use super::*;

#[test]
pub fn test1() {
    assert_eq!("1", "1");
}

#[test]
pub fn test2() {
    assert_ne!("1", "2");
}

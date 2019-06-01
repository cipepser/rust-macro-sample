#![allow(unused_macros)]

macro_rules! basic {
    () => { "basic" };
}

#[test]
fn test_basic() {
    assert_eq!(basic!(), "basic");
}

macro_rules! patterns {
    (cat) => { "cat" };
    (dog) => { "dog" };
    ({}) => { "brace" };
}

#[test]
fn test_patterns() {
    assert_eq!(patterns!(cat), "cat");
    assert_eq!(patterns!(dog), "dog");
    assert_eq!(patterns!({}), "brace");
}
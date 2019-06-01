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

macro_rules! define_fn {
    ($f: ident, $ret: ty, $val: expr) => {
        fn $f() -> $ret { $val }
    };
}

#[test]
fn test_define_fn() {
    define_fn!(six, i32, 1 + 2 + 3);
    assert_eq!(six(), 6);

    define_fn!(hello, String, "hello".to_string());
    assert_eq!(hello(), "hello");
}
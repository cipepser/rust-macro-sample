#![allow(unused_macros)]

macro_rules! basic {
    () => { "basic" };
}

#[test]
fn test_basic() {
    assert_eq!(basic!(), "basic");
}
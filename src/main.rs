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

macro_rules! parse_tt {
    ($t1: tt) => { format!("1. {}", stringify!($t1)) };
    ($t1: tt $t2: tt) => { format!("1. {}, 2. {}", stringify!($t1), stringify!($t2)) };
    ($t1: tt $t2: tt $t3: tt) => { format!("1. {}, 2. {}, 3. {}", stringify!($t1), stringify!($t2), stringify!($t3)) };
}

#[test]
fn test_parse_tt() {
    assert_eq!(parse_tt!(100), "1. 100");
    assert_eq!(parse_tt!(aaa bbb), "1. aaa, 2. bbb");
    assert_eq!(parse_tt!(true + false), "1. true, 2. +, 3. false");
    assert_eq!(parse_tt!([1, 2, 3]), "1. [ 1 , 2 , 3 ]");
    assert_eq!(parse_tt!(
        struct User {
            name: String,
            age: i32,
        }
    ), "1. struct, 2. User, 3. { name : String , age : i32 , }");
}

macro_rules! repeat {
    ( $($e: expr),* ) => { &[ $( $e ),* ] };
    ( $($e: expr);* ) => { repeat!( $( $e ),* ) };
}

#[test]
fn test_repeat() {
    let v: &[i32] = repeat!(10, 20, 30);
    assert_eq!(format!("{:?}", v), "[10, 20, 30]");

    let v: &[i32] = repeat!(100; 200; 300);
    assert_eq!(format!("{:?}", v), "[100, 200, 300]");
}
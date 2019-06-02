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

macro_rules! complex_repeat {
    (
        $(
            $x: expr => [
                $( $y: expr ),*
            ]
        )*
    ) => {
        &[
            $(
                $( $x + $y ),*
            ),*
        ]
    };

    (
        $(
            $x: expr;
            $( -> $y: expr )*
            $( => $z: expr )*
        )*
    ) => {
        &[
            $(
                $x
                $( - $z )*
                $( + $y )*
            ),*
        ]
    };
}

#[test]
fn test_complex_repeat() {
    let v: &[i32] = complex_repeat! {
        100 => [1, 2, 3]
        200 => [4, 5, 6]
    };
    assert_eq!(format!("{:?}", v), "[101, 102, 103, 204, 205, 206]");

    let v: &[i32] = complex_repeat! {
      100;
      -> 1
      => 10

      200;
      -> 2
      -> 3
      => 10
      => 20

      300;

      400;
      => 30
    };
    assert_eq!(format!("{:?}", v), "[91, 175, 300, 370]");
}

macro_rules! match_exprs {
    ($($exprs:expr),*) => {
        ( $($exprs),* )
    };

    ($($exprs:expr),* $(,)*) => {
        match_exprs!( $($exprs),* )
    };
}

#[test]
fn test_match_exprs() {
    let t = match_exprs!( 1, "hello", true );
    assert_eq!(format!("{:?}", t), r#"(1, "hello", true)"#);

    let t = match_exprs! {
        true,
        false,
    };
    assert_eq!(format!("{:?}", t), r#"(true, false)"#);
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

macro_rules! tuple_default {
    ($name:ident, $($tup_tys:ty),*) => {
        let $name: ( $( $tup_tys ),* ) = (
            $(
                replace_expr!(
                    ($tup_tys)
                    Default::default()
                )
            ),*
        );
    };
}

#[test]
fn test_tuple_default() {
    tuple_default!(t, i32, String, bool);
    assert_eq!(format!("{:?}", t), r#"(0, "", false)"#);
}

macro_rules! sum_tt {
    ( $t: tt $( $u: tt )+ ) => {
        sum_tt!($t) $( + sum_tt! ( $u ) )*
    };

    ( ( $( $t: tt)+ ) ) => {
        sum_tt!( $( $t )* )
    };

    ( [ $( $t: tt)+ ] ) => {
        sum_tt!( $( $t )* )
    };

    ( { $( $t: tt)+ } ) => {
        sum_tt!( $( $t )* )
    };

    ( $t: tt ) => { stringify!($t).parse::<i32>().unwrap_or_default() };

}

#[test]
fn test_sum_tt() {
    assert_eq!(sum_tt!( 1 2 3 ), 6);
    assert_eq!(sum_tt!( 4 (5 6) ), 15);
    assert_eq!(sum_tt!( [7] 8 {9} (10 11) ), 45);
    assert_eq!(sum_tt!( 1 true 2 foo bar 3 + pi ), 6);
}
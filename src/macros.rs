/// Creates `String` object from
/// passed object of ANY type.
///
/// Based on a [`format`] macro.
#[macro_export]
macro_rules! string {
    ($val:expr) => {
        format!("{}", $val)
    };
}

/// Creates `&str` object from
/// passed object of **ANY** type.
///
/// Based on a [`string`] macro.
#[macro_export]
macro_rules! str {
    ($val:expr) => {
        Box::leak(
            $crate::string!($val)
                .into_boxed_str()
        ) as &str
    };
}

/// Tests whatever the all macros in this file works.
#[test]
fn test_macros() {
    assert_eq!(string!("Hello world!"), String::from("Hello world!"));
    assert_eq!(str!("Hello world!"), "Hello world!");
}

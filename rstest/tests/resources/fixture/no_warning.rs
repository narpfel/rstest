use rstest::*;

#[rustfmt::skip]
#[fixture]
fn string() -> &'static str { "abc" }

#[rstest]
fn test_string(string: &'static str) {
    assert_eq!(string, "abc");
}

#[fixture]
fn val() -> i32 {
    21
}

#[fixture]
fn fortytwo(mut val: i32) -> i32 {
    val *= 2;
    val
}

#[rstest]
fn the_test(fortytwo: i32) {
    assert_eq!(fortytwo, 42);
}

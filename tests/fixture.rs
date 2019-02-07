pub mod prj;
pub mod utils;
pub mod root;

use crate::utils::{*, deindent::Deindent};
use crate::prj::Project;

fn prj(res: &str) -> Project {
    root::prj()
        .set_code_file(resources(res))
}

fn run_test(res: &str) -> std::process::Output {
    prj(res).run_tests()
        .unwrap()
}

#[test]
fn happy_path_one_success_and_one_fail() {
    let output = run_test("fixture_simple.rs");

    TestResults::new()
        .ok("should_success")
        .fail("should_fail")
        .assert(output);
}

#[test]
fn mutable_fixture() {
    let output = run_test("fixture_mut.rs");

    TestResults::new()
        .ok("should_success")
        .fail("should_fail")
        .assert(output);
}

#[test]
fn should_panic() {
    let output = run_test("fixture_panic.rs");

    TestResults::new()
        .ok("should_success")
        .fail("should_fail")
        .assert(output);
}

mod dump_fixture_value {
    use super::{run_test, TestResults, utils::Stringable, assert_in};

    #[test]
    fn dump_it_if_implements_debug() {
        let output = run_test("fixture_dump_debug.rs");
        let out = output.stdout.str().to_string();

        TestResults::new()
            .fail("should_fail")
            .assert(output);

        assert_in!(out, "fu32 = 42");
        assert_in!(out, r#"fstring = "A String""#);
        assert_in!(out, r#"ftuple = (A, "A String", -12"#);
    }

    #[test]
    fn not_compile_if_not_implement_debug() {
        let output = run_test("fixture_dump_not_debug.rs");

        let out = output.stderr.str().to_string();

        assert_in!(out, "method `display_string` not found for this");
    }

    #[test]
    fn exclude_some_fixtures() {
        let output = run_test("fixture_dump_exclude_some_fixtures.rs");
        let out = output.stdout.str().to_string();

        TestResults::new()
            .fail("should_fail")
            .assert(output);

        assert_in!(out, "fu32 = 42");
        assert_in!(out, "fd = D");
    }

    #[test]
    fn fixture_values_should_be_after_test_arguments_and_before_test_start() {
        let output = run_test("fixture_dump_exclude_some_fixtures.rs");
        let out = output.stdout.str().to_string();

        TestResults::new()
            .fail("should_fail")
            .assert(output);

        let fixture_dumps_lines = out.lines()
            .skip_while(|l|
                !l.contains("TEST ARGUMENTS"))
            .take_while(|l|
                !l.contains("TEST START"))
            .collect::<Vec<_>>();

        assert_eq!(3, fixture_dumps_lines.len());
    }

}

#[test]
fn should_show_correct_errors() {
    let prj = prj("fixture_errors.rs");
    let output = prj.run_tests().unwrap();
    let name = prj.get_name();

    assert_in!(output.stderr.str(), format!("
        error[E0425]: cannot find function `no_fixture` in this scope
          --> {}/src/lib.rs:10:1
           |
        10 | #[rstest]", name).deindent());

    assert_in!(output.stderr.str(), format!(r#"
        error[E0308]: mismatched types
         --> {}/src/lib.rs:7:18
          |
        7 |     let a: u32 = "";
          |                  ^^ expected u32, found reference
          |
          = note: expected type `u32`
                     found type `&'static str`
        "#, name).deindent());

    assert_in!(output.stderr.str(), format!("
        error[E0308]: mismatched types
          --> {}/src/lib.rs:15:29
           |
        15 | fn error_fixture_wrong_type(fixture: String) {{
           |                             ^^^^^^^
           |                             |
           |                             expected struct `std::string::String`, found u32
           |                             help: try using a conversion method: `fixture.to_string()`
           |
           = note: expected type `std::string::String`
                      found type `u32`
        ", name).deindent());
}

#[test]
fn should_reject_no_item_function() {
    let prj = prj("fixture_reject_no_item_function.rs");
    let output = prj.compile().unwrap();
    let name = prj.get_name();

    assert_in!(output.stderr.str(), format!("
        error: expected `fn`
         --> {}/src/lib.rs:4:1
          |
        4 | struct Foo;
          | ^^^^^^
        ", name).deindent());

    assert_in!(output.stderr.str(), format!("
        error: expected `fn`
         --> {}/src/lib.rs:7:1
          |
        7 | impl Foo {{}}
          | ^^^^
        ", name).deindent());

    assert_in!(output.stderr.str(), format!("
        error: expected `fn`
          --> {}/src/lib.rs:10:1
           |
        10 | mod mod_baz {{}}
           | ^^^
        ", name).deindent());
}

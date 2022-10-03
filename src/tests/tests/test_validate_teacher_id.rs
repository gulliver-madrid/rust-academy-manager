#![cfg(test)]

use pretty_assertions::assert_eq;
use rust_i18n::t;

use crate::{menus::validate_teacher_id, tests::helpers::highlight};

struct Case(Option<String>, Result<u32, String>);

#[test]
fn test_validate_teacher_id() {
    let cases = [
        Case(
            Some("123".to_string()), // fmt
            Ok(123),
        ),
        Case(
            Some("   123".to_string()), // fmt
            Ok(123),
        ),
        Case(
            Some("123".to_string()), // fmt
            Ok(123),
        ),
        Case(
            Some("   123".to_string()), // fmt
            Ok(123),
        ),
        Case(
            Some("-123".to_string()),
            Err(t!("id_should_have_only_digits")),
        ),
        Case(
            Some("hello 123".to_string()),
            Err(t!("id_should_have_only_digits")),
        ),
        Case(
            Some("Michael".to_string()),
            Err(t!("teacher_id_should_be_a_number")),
        ),
        Case(
            Some("7777777777777777".to_string()), // fmt
            Err(t!("no_valid_id")),
        ),
        Case(
            Some("    ".to_string()), // fmt
            Err(t!("no_id_was_entered")),
        ),
        Case(
            None, // fmt
            Err(t!("no_id_was_entered")),
        ),
    ];
    for case in cases {
        assert_validation_is_correct(case)
    }
}

fn assert_validation_is_correct(case: Case) {
    let Case(user_input, expected) = case;
    let result = validate_teacher_id(&user_input);
    assert_eq!(
        result,
        expected,
        "{}",
        highlight(&format!("User input: {:?}", user_input))
    );
}

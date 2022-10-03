#![cfg(test)]

use crate::helpers;
use pretty_assertions::assert_eq;

#[test]
fn get_last_element() {
    let vector = vec![1, 2, 3, 4, 5];
    let res = helpers::get_last_element(&vector);
    assert_eq!(res, Some(&5));
}

#[test]
fn only_contains_digits() {
    let numeric = "0123456";
    let not_numerics = ["hello", "0123 hola 456"];
    assert_eq!(helpers::only_contains_digits(numeric), true);
    for s in not_numerics {
        assert_eq!(helpers::only_contains_digits(s), false);
    }
}
#[test]
fn contains_some_digits() {
    let has_digits = ["0123456", "0123 hola 456"];
    let has_no_digits = "hello";
    for s in has_digits {
        assert_eq!(helpers::contains_some_digits(s), true);
    }
    assert_eq!(helpers::contains_some_digits(has_no_digits), false);
}

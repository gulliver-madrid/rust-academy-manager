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
fn is_numeric() {
    let numeric = "0123456";
    let not_numerics = ["hello", "0123 hola 456"];
    assert_eq!(helpers::is_numeric(numeric), true);
    for s in not_numerics {
        assert_eq!(helpers::is_numeric(s), false);
    }
}

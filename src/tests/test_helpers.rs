#[cfg(test)] // Compiler only detect this statement in test mode
use crate::helpers;

#[test]
fn get_last_element() {
    let vector = vec![1, 2, 3, 4, 5];
    let res = helpers::get_last_element(&vector);
    assert_eq!(res, Some(&5));
}

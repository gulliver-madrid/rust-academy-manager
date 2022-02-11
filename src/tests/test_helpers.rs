#[cfg(test)] // El compilador solo lo detecta en modo test
use crate::helpers;

#[test]
fn get_last_element() {
    let vector = vec![1, 2, 3, 4, 5];
    let res = helpers::get_last_element(&vector);
    assert_eq!(res, Some(&5));
}

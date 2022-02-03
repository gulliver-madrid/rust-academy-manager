#[cfg(test)] // El compilador solo lo detecta en modo test
fn add(x: i8, y: i8) -> i8 {
    x + y
}

#[test]
fn add_works() {
    println!("Ejecutando el primer test");
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(5, -2), 3);
}
#[test]
fn add_works_too() {
    println!("Ejecutando el segundo test");
    assert_eq!(add(2, 2), 4);
}

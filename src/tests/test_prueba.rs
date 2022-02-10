use crate::consola::InnerConsole;

#[cfg(test)]
use crate::{
    application::Application,
    components::Control,
    consola::{self},
    menus::{Menu, MenuPrincipal},
};

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

pub struct MockConsole {}

impl InnerConsole for MockConsole {
    fn clear_screen(&self) {}
    fn get_input(&self) -> String {
        String::from("4") // El número necesario para salir del menu
    }
    fn mostrar(&self, _texto: &str) {}
}

#[test]
fn mocking() {
    println!("Falta implementar el mocking test");
    let application = Application::new();
    let consola = consola::Consola {
        inner_console: Box::new(MockConsole {}),
    };
    let mut control = Control {
        consola,
        application,
    };
    let mut menu = MenuPrincipal {};
    menu.abrir_menu(&mut control);
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(2, 2), 4);
}

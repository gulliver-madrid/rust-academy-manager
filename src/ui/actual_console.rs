use super::inner_console::InnerConsole;
use std::io;

const USAR_BORRADO: bool = true;

pub struct ActualConsole {}

impl InnerConsole for ActualConsole {
    fn clear_screen(&self) {
        if USAR_BORRADO {
            clearscreen::clear().unwrap();
        }
    }
    fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error: no se pudo leer la entrada del usuario");
        String::from(input.trim())
    }
    fn mostrar(&self, texto: &str) {
        println!("{}", texto);
    }
}

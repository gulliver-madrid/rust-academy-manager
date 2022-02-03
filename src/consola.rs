use crate::helpers;
use std::io;

const USAR_BORRADO: bool = true;

pub struct Consola;

impl Consola {
    pub fn clear_screen(&self) {
        if USAR_BORRADO {
            clearscreen::clear().unwrap();
        }
    }
    pub fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error: no se pudo leer la entrada del usuario");
        String::from(input.trim())
    }

    pub fn pide_texto_a_usuario(&self) -> Option<String> {
        match self.get_input().trim() {
            "" => None,
            texto => Some(String::from(texto)),
        }
    }

    pub fn mostrar(&self, texto: &str) {
        println!("{}", texto);
    }

    pub fn mostrar_titulo(&self, texto: &str) {
        let chars = texto.chars();
        let n = chars.count();
        let mut s = String::new();
        s.push('\n');
        s.push_str(texto);
        s.push('\n');
        let mut s = helpers::add_repeated_char(s, '-', n);
        s.push('\n');
        self.mostrar(&s);
    }
}

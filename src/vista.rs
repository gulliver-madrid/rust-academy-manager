use std::io;

use crate::helpers;

pub struct Vista;

impl Vista {
    pub fn clear_screen(&self) {
        clearscreen::clear().unwrap();
    }
    pub fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        String::from(input.trim())
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

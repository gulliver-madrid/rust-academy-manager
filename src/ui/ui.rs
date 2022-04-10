use crate::helpers;

use crate::menus::shared::MenuItem;

use super::inner_console::InnerConsole;

pub struct UserInterface {
    pub inner_console: Box<dyn InnerConsole>,
}

impl UserInterface {
    pub fn get_user_choice<'a, T>(
        &self,
        menu_items: &'a Vec<MenuItem<T>>,
    ) -> Option<&'a T> {
        // T es una enum que representa una opcion de menu
        let eleccion = self.get_input();
        crate::menus::shared::extract_option(eleccion, &menu_items)
    }

    pub fn clear_screen(&self) {
        self.inner_console.clear_screen();
    }

    pub fn get_input(&self) -> String {
        self.inner_console.get_input()
    }

    pub fn ask_text_to_user(&self) -> Option<String> {
        match self.get_input().trim() {
            "" => None,
            texto => Some(String::from(texto)),
        }
    }

    pub fn show(&self, texto: &str) {
        self.inner_console.show(texto);
    }

    pub fn show_title(&self, texto: &str) {
        self.show(&self.convert_to_title(texto));
    }

    pub fn pause_enter(&self, texto: &str) {
        self.show(&format!("Pulsa ENTER para {}", texto));
        self.get_input();
    }

    /// Anade una linea de subrayado al texto especificado
    fn convert_to_title(&self, texto: &str) -> String {
        let chars = texto.chars();
        let n = chars.count();
        let mut s = String::new();
        s.push('\n');
        s.push_str(texto);
        s.push('\n');
        let mut s = helpers::add_repeated_char(s, '-', n);
        s.push('\n');
        s
    }
}

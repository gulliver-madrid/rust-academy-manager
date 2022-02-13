use crate::helpers;

use crate::menus::shared::ItemMenu;

use super::inner_console::InnerConsole;

pub struct UserInterface {
    pub inner_console: Box<dyn InnerConsole>,
}

impl UserInterface {
    pub fn get_user_choice<'a, T>(
        &self,
        items_menu: &'a Vec<ItemMenu<T>>,
    ) -> Option<&'a T> {
        // T es una enum que representa una opcion de menu
        let eleccion = self.get_input();
        crate::menus::shared::extraer_opcion(eleccion, &items_menu)
    }

    pub fn clear_screen(&self) {
        self.inner_console.clear_screen();
    }

    pub fn get_input(&self) -> String {
        self.inner_console.get_input()
    }

    pub fn pide_texto_a_usuario(&self) -> Option<String> {
        match self.get_input().trim() {
            "" => None,
            texto => Some(String::from(texto)),
        }
    }

    pub fn mostrar(&self, texto: &str) {
        self.inner_console.mostrar(texto);
    }

    pub fn mostrar_titulo(&self, texto: &str) {
        self.mostrar(&self.convertir_a_titulo(texto));
    }

    pub fn pausa_enter(&self, texto: &str) {
        self.mostrar(&format!("Pulsa ENTER para {}", texto));
        self.get_input();
    }

    /// Anade una linea de subrayado al texto especificado
    fn convertir_a_titulo(&self, texto: &str) -> String {
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

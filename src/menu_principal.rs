use crate::menu_profes;
use crate::vista;

pub struct MenuPrincipal;

impl MenuPrincipal {
    pub fn abrir_menu(&self) {
        let vista = vista::Vista {};
        loop {
            vista.mostrar(
                "
Menú Principal
---------------

Elige una opción:
1 - Profesores
2 - Salir
",
            );
            let nombre = vista.get_input();
            let eleccion = nombre.trim();
            match eleccion {
                "1" => self.abrir_menu_profes(&vista),
                "2" => return,
                _ => continue,
            }
        }
    }

    fn abrir_menu_profes(&self, vista: &vista::Vista) {
        let menu = menu_profes::MenuProfesores {};
        menu.abrir_menu(&vista);
    }
}

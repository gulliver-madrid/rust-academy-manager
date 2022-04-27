use crate::menu_asignaturas;
use crate::menu_profes;
use crate::textos;
use crate::vista;

pub struct MenuPrincipal;

impl MenuPrincipal {
    pub fn abrir_menu(&self) {
        let vista = vista::Vista {};
        loop {
            vista.clear_screen();
            vista.mostrar(textos::OPCIONES_MENU_PRINCIPAL);
            let nombre = vista.get_input();
            let eleccion = nombre.trim();
            match eleccion {
                "1" => self.abrir_menu_profes(&vista),
                "2" => self.abrir_menu_asignaturas(&vista),
                "3" => return,
                _ => continue,
            }
        }
    }

    fn abrir_menu_profes(&self, vista: &vista::Vista) {
        let menu = menu_profes::MenuProfesores {};
        menu.abrir_menu(&vista);
    }

    fn abrir_menu_asignaturas(&self, vista: &vista::Vista) {
        let menu = menu_asignaturas::MenuAsignaturas {};
        menu.abrir_menu(&vista);
    }
}

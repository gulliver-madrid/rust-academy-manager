use crate::menu_asignaturas;
use crate::menu_profes;
use crate::shared_menus as menus;
use crate::textos;
use crate::vista;

enum Opcion {
    Profesores,
    Asignaturas,
    Salir,
}

const ITEMS_MENU_DATA: [(&Opcion, menus::TextoOpcion); 3] = [
    (&Opcion::Profesores, "Profesores"),
    (&Opcion::Asignaturas, "Asignaturas"),
    (&Opcion::Salir, "Salir"),
];

pub struct MenuPrincipal<'a> {
    pub vista: &'a vista::Vista,
}

impl<'a> MenuPrincipal<'a> {
    pub fn abrir_menu(&self) {
        let items_menu = menus::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            self.vista.clear_screen();
            self.vista.mostrar(textos::TITULO_MENU_PRINCIPAL);
            let opciones_string = menus::crear_texto_opciones(&items_menu);
            self.vista.mostrar(&opciones_string);

            let eleccion = self.vista.get_input();
            let posible_opcion = menus::extraer_opcion(eleccion, &items_menu);
            if let Some(opcion) = posible_opcion {
                match opcion {
                    Opcion::Asignaturas => self.abrir_menu_asignaturas(self.vista),
                    Opcion::Profesores => self.abrir_menu_profes(self.vista),
                    Opcion::Salir => return,
                }
            } else {
                continue;
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

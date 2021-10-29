use crate::menu_asignaturas;
use crate::menu_profes;
use crate::shared_menus::{extraer_opcion, ItemMenu};
use crate::textos;
use crate::vista;

enum Opcion {
    Profesores,
    Asignaturas,
    Salir,
}

pub struct MenuPrincipal;

impl MenuPrincipal {
    pub fn abrir_menu(&self) {
        let vista = vista::Vista {};
        let items_menu = vec![
            ItemMenu {
                texto: "Profesores",
                opcion: Opcion::Profesores,
            },
            ItemMenu {
                texto: "Asignaturas",
                opcion: Opcion::Asignaturas,
            },
            ItemMenu {
                texto: "Salir",
                opcion: Opcion::Salir,
            },
        ];

        loop {
            vista.clear_screen();
            vista.mostrar(textos::TITULO_MENU_PRINCIPAL);

            for (i, item) in items_menu.iter().enumerate() {
                vista.mostrar(&format!("{} - {}", i + 1, item.texto))
            }

            let eleccion = vista.get_input();
            let posible_opcion = extraer_opcion(eleccion, &items_menu);
            if let Some(opcion) = posible_opcion {
                match opcion {
                    Opcion::Asignaturas => self.abrir_menu_asignaturas(&vista),
                    Opcion::Profesores => self.abrir_menu_profes(&vista),
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

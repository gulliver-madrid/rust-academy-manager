use crate::menu_asignaturas;
use crate::menu_profes;
use crate::textos;
use crate::vista;

pub enum OpcionMenuPrincipal {
    Profesores,
    Asignaturas,
    Salir,
}

pub struct ItemMenu<'a, T> {
    pub texto: &'a str,
    pub opcion: T,
}

pub struct MenuPrincipal;

fn extraer_opcion<'a, OpcionMenu>(
    eleccion: String,
    items_menu: &'a Vec<ItemMenu<OpcionMenu>>,
) -> Option<&'a OpcionMenu> {
    let posible_num_opcion: Option<usize> = eleccion.parse().ok();
    if let Some(num_opcion) = posible_num_opcion {
        let posible_item_menu = items_menu.get(num_opcion - 1);
        match posible_item_menu {
            Some(item_elegido) => Some(&item_elegido.opcion),
            None => None,
        }
    } else {
        None
    }
}

impl MenuPrincipal {
    pub fn abrir_menu(&self) {
        let vista = vista::Vista {};
        let items_menu = vec![
            ItemMenu {
                texto: "Profesores",
                opcion: OpcionMenuPrincipal::Profesores,
            },
            ItemMenu {
                texto: "Asignaturas",
                opcion: OpcionMenuPrincipal::Asignaturas,
            },
            ItemMenu {
                texto: "Salir",
                opcion: OpcionMenuPrincipal::Salir,
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
                    OpcionMenuPrincipal::Asignaturas => {
                        self.abrir_menu_asignaturas(&vista)
                    }
                    OpcionMenuPrincipal::Profesores => self.abrir_menu_profes(&vista),
                    OpcionMenuPrincipal::Salir => return,
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

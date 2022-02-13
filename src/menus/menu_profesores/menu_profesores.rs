use super::menu_anadir_profesor;
use super::menu_eliminar_profesor::MenuEliminarProfesor;

use crate::components::Control;
use crate::menus::shared;
use crate::menus::shared::{ItemMenu, SalirMenu, TextoOpcion};
use crate::textos;
use crate::views::View;

#[derive(Clone)]
enum Opcion {
    MostrarLista,
    AnadirProfesor,
    EliminarProfesor,
    Volver,
}
type ItemMenus<'a> = Vec<ItemMenu<'a, Opcion>>;

const ITEMS_MENU_DATA: [(Opcion, TextoOpcion); 4] = [
    (Opcion::MostrarLista, "Ver la lista de profesores"),
    (Opcion::AnadirProfesor, "Añadir un profesor"),
    (Opcion::EliminarProfesor, "Eliminar un profesor"),
    (Opcion::Volver, "Volver al menú principal"),
];

pub struct MenuProfesores<'a> {
    control: &'a mut Control,
}

impl<'a> MenuProfesores<'_> {
    pub fn new(control: &mut Control) -> MenuProfesores {
        MenuProfesores { control }
    }

    pub fn abrir_menu(&mut self) {
        self.control.application.teachers_app.load_profesores();
        let items_menu = shared::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            match self.mostrar_iteracion_menu(&items_menu) {
                Some(SalirMenu) => {
                    break;
                }
                _ => continue,
            }
        }
    }
    fn mostrar_iteracion_menu(
        &mut self,
        items_menu: &ItemMenus,
    ) -> Option<SalirMenu> {
        self.mostrar_texto_menu(items_menu);
        let opcion_elegida = self.control.ui.get_user_choice(&items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_profesores(),
            Opcion::AnadirProfesor => self.abrir_menu_anadir_profesor(),
            Opcion::EliminarProfesor => self.abrir_menu_eliminar_profesor(),
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_texto_menu(&self, items_menu: &ItemMenus) {
        let ui = &self.control.ui;
        ui.clear_screen();
        ui.mostrar_titulo(textos::MENU_PROFESORES);
        let texto_opciones = shared::crear_texto_opciones(&items_menu);
        ui.mostrar(&texto_opciones);
    }
    fn mostrar_lista_profesores(&self) {
        let profesores = self.control.application.teachers_app.get_teachers();
        let ui = &self.control.ui;
        ui.clear_screen();
        ui.mostrar_titulo(textos::LISTA_PROFESORES);
        for profesor in profesores {
            ui.mostrar(&profesor.crear_linea_tabla());
        }
        ui.pausa_enter("volver al menú de profesores");
    }

    fn abrir_menu_anadir_profesor(&mut self) {
        let mut menu = menu_anadir_profesor::MenuAnadirProfesor {
            control: self.control,
        };
        menu.abrir_menu();
    }

    fn abrir_menu_eliminar_profesor(&mut self) {
        let mut menu = MenuEliminarProfesor {
            control: self.control,
        };
        menu.abrir_menu();
    }
}

use super::menu_eliminar_asignatura::MenuEliminarAsignatura;

use crate::components::Control;
use crate::ui::UserInterface;

use crate::dominio::Asignaturas;
use crate::menus::menu_asignar_profesor::MenuAsignarProfesor;
use crate::menus::shared::{self, ItemMenu, SalirMenu};
use crate::textos;
use crate::views::View;

use super::menu_anadir_asignatura::MenuAnadirAsignatura;

#[derive(Clone)]
enum Opcion {
    MostrarLista,
    AnadirAsignatura,
    EliminarAsignatura,
    AsignarProfesor,
    Volver,
}

type ItemsMenu<'a> = Vec<ItemMenu<'a, Opcion>>;

const ITEMS_MENU_DATA: [(Opcion, shared::TextoOpcion); 5] = [
    (Opcion::MostrarLista, "Ver la lista de asignaturas"),
    (Opcion::AnadirAsignatura, "Añadir una asignatura"),
    (Opcion::EliminarAsignatura, "Eliminar una asignatura"),
    (
        Opcion::AsignarProfesor,
        "Asignar un profesor a una asignatura",
    ),
    (Opcion::Volver, "Volver al menú principal"),
];

pub struct MenuAsignaturas<'a> {
    pub control: &'a mut Control,
}

impl MenuAsignaturas<'_> {
    pub fn new(control: &mut Control) -> MenuAsignaturas {
        MenuAsignaturas { control }
    }
    pub fn abrir_menu(&mut self) {
        self.control.application.load_subjects();
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
        items_menu: &ItemsMenu,
    ) -> Option<SalirMenu> {
        let ui = &self.control.ui;
        self.mostrar_texto_menu(items_menu, ui);

        let opcion_elegida = ui.get_user_choice(&items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_asignaturas(),
            Opcion::AnadirAsignatura => self.abrir_menu_anadir_asignatura(),
            Opcion::EliminarAsignatura => self.abrir_menu_eliminar_asignatura(),
            Opcion::AsignarProfesor => {
                self.abrir_menu_asignar_profesor_a_asignatura()
            }
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }

    fn mostrar_texto_menu(&self, items_menu: &ItemsMenu, ui: &UserInterface) {
        ui.clear_screen();
        ui.mostrar_titulo(textos::MENU_ASIGNATURAS);
        let texto_opciones = shared::crear_texto_opciones(&items_menu);
        ui.mostrar(&texto_opciones);
    }

    fn mostrar_lista_asignaturas(&self) {
        let ui = &self.control.ui;
        let asignaturas = self.control.application.get_asignaturas();
        match asignaturas {
            Ok(asignaturas) => {
                let texto_lista_asignaturas =
                    self.crear_lista_asignaturas(asignaturas);
                ui.clear_screen();
                ui.mostrar_titulo(textos::LISTA_ASIGNATURAS);
                ui.mostrar(texto_lista_asignaturas.as_str());
            }
            Err(e) => ui.mostrar(&e.to_string()),
        }
        ui.pausa_enter("volver al menú de asignaturas");
    }

    fn crear_lista_asignaturas(&self, asignaturas: Asignaturas) -> String {
        asignaturas
            .iter()
            .map(|asignatura| asignatura.crear_linea_tabla())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn abrir_menu_anadir_asignatura(&mut self) {
        MenuAnadirAsignatura {
            control: self.control,
        }
        .abrir_menu();
    }
    fn abrir_menu_eliminar_asignatura(&mut self) {
        MenuEliminarAsignatura {
            control: self.control,
        }
        .abrir_menu();
    }
    fn abrir_menu_asignar_profesor_a_asignatura(&mut self) {
        MenuAsignarProfesor {
            control: self.control,
        }
        .abrir_menu();
    }
}

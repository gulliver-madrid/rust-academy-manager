use shared::Menu;

use super::menu_eliminar_asignatura::MenuEliminarAsignatura;

use crate::components::{Component, Control};
use crate::consola::Consola;

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

type ItemMenus<'a> = Vec<ItemMenu<'a, Opcion>>;

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

pub struct MenuAsignaturas {}

impl Component for MenuAsignaturas {
    fn render(&mut self, control: &mut Control) {
        self.abrir_menu(control);
    }
}
impl Menu for MenuAsignaturas {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

impl MenuAsignaturas {
    fn _abrir_menu(&mut self, control: &mut Control) {
        control.application.load_subjects();
        let items_menu = shared::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            match self.mostrar_iteracion_menu(&items_menu, control) {
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
        control: &mut Control,
    ) -> Option<SalirMenu> {
        let consola = &control.consola;
        self.mostrar_texto_menu(items_menu, consola);
        let entrada_usuario = consola.get_input();
        let opcion_elegida = shared::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_asignaturas(control),
            Opcion::AnadirAsignatura => self.abrir_menu_anadir_asignatura(control),
            Opcion::EliminarAsignatura => {
                self.abrir_menu_eliminar_asignatura(control)
            }
            Opcion::AsignarProfesor => {
                self.abrir_menu_asignar_profesor_a_asignatura(control)
            }
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }

    fn mostrar_texto_menu(&self, items_menu: &ItemMenus, consola: &Consola) {
        consola.clear_screen();
        consola.mostrar_titulo(textos::MENU_ASIGNATURAS);
        let texto_opciones = shared::crear_texto_opciones(&items_menu);
        consola.mostrar(&texto_opciones);
    }

    fn mostrar_lista_asignaturas(&self, control: &mut Control) {
        let consola = &control.consola;
        let asignaturas = control.application.get_asignaturas();
        match asignaturas {
            Ok(asignaturas) => {
                let texto_lista_asignaturas =
                    self.crear_lista_asignaturas(asignaturas);
                consola.clear_screen();
                consola.mostrar_titulo(textos::LISTA_ASIGNATURAS);
                consola.mostrar(texto_lista_asignaturas.as_str());
            }
            Err(e) => consola.mostrar(&e.to_string()),
        }
        consola.pausa_enter("volver al menú de asignaturas");
    }

    fn crear_lista_asignaturas(&self, asignaturas: Asignaturas) -> String {
        asignaturas
            .iter()
            .map(|asignatura| asignatura.crear_linea_tabla())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn abrir_menu_anadir_asignatura(&mut self, control: &mut Control) {
        MenuAnadirAsignatura {}.abrir_menu(control);
    }
    fn abrir_menu_eliminar_asignatura(&mut self, control: &mut Control) {
        MenuEliminarAsignatura {}.abrir_menu(control);
    }
    fn abrir_menu_asignar_profesor_a_asignatura(&mut self, control: &mut Control) {
        MenuAsignarProfesor {}.abrir_menu(control);
    }
}

use super::menu_asignar_profesor::MenuAsignarProfesor;
use super::menu_eliminar_asignatura::MenuEliminarAsignatura;
use super::shared as menus;
use super::shared::Menu;
use super::shared::{ItemMenu, SalirMenu};

use crate::components::{Component, Control};
use crate::dominio::asignatura::Asignaturas;

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

const ITEMS_MENU_DATA: [(Opcion, menus::TextoOpcion); 5] = [
    (Opcion::MostrarLista, "Ver la lista de asignaturas"),
    (Opcion::AnadirAsignatura, "Añadir una asignatura"),
    (Opcion::EliminarAsignatura, "Eliminar una asignatura"),
    (
        Opcion::AsignarProfesor,
        "Asignar un profesor a una asignatura",
    ),
    (Opcion::Volver, "Volver al menú principal"),
];

pub struct MenuAsignaturas {
    asignaturas: Asignaturas,
}

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
    pub fn new(asignaturas: Asignaturas) -> MenuAsignaturas {
        MenuAsignaturas { asignaturas }
    }

    fn _abrir_menu(&mut self, control: &mut Control) {
        let items_menu = menus::crear_items_menu(ITEMS_MENU_DATA);
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
        self.mostrar_texto_menu(items_menu, control);
        let entrada_usuario = control.consola.get_input();
        let opcion_elegida = menus::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_asignaturas(control),
            Opcion::AnadirAsignatura => self.abrir_menu_anadir_asignatura(control),
            Opcion::EliminarAsignatura => self.abrir_menu_eliminar_asignatura(control),
            Opcion::AsignarProfesor => {
                self.abrir_menu_asignar_profesor_a_asignatura(control)
            }
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }

    fn mostrar_texto_menu(&self, items_menu: &ItemMenus, control: &mut Control) {
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::MENU_ASIGNATURAS);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        control.consola.mostrar(&texto_opciones);
    }

    fn mostrar_lista_asignaturas(&self, control: &mut Control) {
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::LISTA_ASIGNATURAS);
        let texto_lista_asignaturas = self.crear_lista_asignaturas();
        control.consola.mostrar(texto_lista_asignaturas.as_str());
        control.consola.pausa_enter("volver al menú de asignaturas");
    }

    fn crear_lista_asignaturas(&self) -> String {
        self.asignaturas
            .iter()
            .map(|asignatura| asignatura.crear_linea_tabla())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn abrir_menu_anadir_asignatura(&mut self, control: &mut Control) {
        let mut menu = MenuAnadirAsignatura::new(&mut self.asignaturas);
        menu.abrir_menu(control);
    }
    fn abrir_menu_eliminar_asignatura(&mut self, control: &mut Control) {
        let mut menu = MenuEliminarAsignatura::new(&mut self.asignaturas);
        menu.abrir_menu(control);
    }
    fn abrir_menu_asignar_profesor_a_asignatura(&mut self, control: &mut Control) {
        control
            .consola
            .mostrar("Elige asignatura a la que quieras asignar profesor");
        let entrada = control.consola.pide_texto_a_usuario();
        match entrada {
            Some(texto) => {
                match self.asignaturas.iter().position(|a| a.nombre == texto) {
                    Some(index) => {
                        let id_asignatura = self.asignaturas[index].id;
                        let mut menu = MenuAsignarProfesor {
                            asignaturas: &mut self.asignaturas,
                            id_asignatura,
                        };
                        menu.abrir_menu(control);
                    }
                    None => {
                        control
                            .consola
                            .mostrar(&format!("Nombre no válido: {}", texto));
                        control.consola.pausa_enter("continuar");
                    }
                }
            }
            None => {
                return;
            }
        }
    }
}

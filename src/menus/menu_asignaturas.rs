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
    Volver,
}

type ItemMenus<'a> = Vec<ItemMenu<'a, Opcion>>;

const ITEMS_MENU_DATA: [(Opcion, menus::TextoOpcion); 3] = [
    (Opcion::MostrarLista, "Ver la lista de asignaturas"),
    (Opcion::AnadirAsignatura, "Añadir una asignatura"),
    (Opcion::Volver, "Volver al menú principal"),
];

pub struct MenuAsignaturas {
    asignaturas: Asignaturas,
}

impl Component for MenuAsignaturas {
    fn render(&mut self, control: &Control) {
        self.abrir_menu(control);
    }
}
impl Menu for MenuAsignaturas {
    fn abrir_menu(&mut self, control: &Control) {
        self._abrir_menu(control);
    }
}

impl MenuAsignaturas {
    pub fn new(asignaturas: Asignaturas) -> MenuAsignaturas {
        MenuAsignaturas { asignaturas }
    }

    fn _abrir_menu(&mut self, control: &Control) {
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
        control: &Control,
    ) -> Option<SalirMenu> {
        self.mostrar_texto_menu(items_menu, control);
        let entrada_usuario = control.consola.get_input();
        let opcion_elegida = menus::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_asignaturas(control),
            Opcion::AnadirAsignatura => self.abrir_menu_anadir_asignatura(control),
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }

    fn mostrar_texto_menu(&self, items_menu: &ItemMenus, control: &Control) {
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::MENU_ASIGNATURAS);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        control.consola.mostrar(&texto_opciones);
    }

    fn mostrar_lista_asignaturas(&self, control: &Control) {
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::LISTA_ASIGNATURAS);
        let texto_lista_asignaturas = self.crear_lista_asignaturas();
        control.consola.mostrar(texto_lista_asignaturas.as_str());
        control
            .consola
            .mostrar("\nPulsa ENTER para volver al menú de asignaturas");
        control.consola.get_input();
    }

    fn crear_lista_asignaturas(&self) -> String {
        self.asignaturas
            .iter()
            .map(|asignatura| asignatura.crear_linea_tabla())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn abrir_menu_anadir_asignatura(&mut self, control: &Control) {
        let mut menu = MenuAnadirAsignatura::new(&mut self.asignaturas);
        menu.abrir_menu(control);
    }
}

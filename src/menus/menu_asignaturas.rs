use super::shared as menus;
use super::shared::Menu;
use super::shared::{ItemMenu, SalirMenu};
use crate::consola;
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

pub struct MenuAsignaturas<'a> {
    consola: &'a consola::Consola,
    asignaturas: Asignaturas,
}

impl Menu for MenuAsignaturas<'_> {
    fn abrir_menu(&mut self) {
        let items_menu = menus::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            if let Some(_instruccion) = self.mostrar_iteracion_menu(&items_menu) {
                break;
            }
        }
    }
}

impl MenuAsignaturas<'_> {
    pub fn new(consola: &consola::Consola, asignaturas: Asignaturas) -> MenuAsignaturas {
        MenuAsignaturas {
            consola,
            asignaturas,
        }
    }

    fn mostrar_iteracion_menu(&mut self, items_menu: &ItemMenus) -> Option<SalirMenu> {
        self.mostrar_texto_menu(items_menu);
        let entrada_usuario = self.consola.get_input();
        let opcion_elegida = menus::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_asignaturas(),
            Opcion::AnadirAsignatura => self.abrir_menu_anadir_asignatura(),
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }

    fn mostrar_texto_menu(&self, items_menu: &ItemMenus) {
        self.consola.clear_screen();
        self.consola.mostrar_titulo(textos::MENU_ASIGNATURAS);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        self.consola.mostrar(&texto_opciones);
    }

    fn mostrar_lista_asignaturas(&self) {
        self.consola.clear_screen();
        self.consola.mostrar_titulo(textos::LISTA_ASIGNATURAS);
        let texto_lista_asignaturas = self.crear_lista_asignaturas();
        self.consola.mostrar(texto_lista_asignaturas.as_str());
        self.consola
            .mostrar("\nPulsa ENTER para volver al menú de asignaturas");
        self.consola.get_input();
    }

    fn crear_lista_asignaturas(&self) -> String {
        self.asignaturas
            .iter()
            .map(|asignatura| asignatura.crear_linea_tabla())
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn abrir_menu_anadir_asignatura(&mut self) {
        let mut menu = MenuAnadirAsignatura::new(self.consola, &mut self.asignaturas);
        menu.abrir_menu();
    }
}

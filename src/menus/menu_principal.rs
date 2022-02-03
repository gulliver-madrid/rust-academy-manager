use super::menu_asignaturas;
use super::menu_profes;
use super::shared as menus;
use super::shared::Menu;
use super::shared::{ItemMenu, SalirMenu};
use crate::consola;
use crate::repo;
use crate::textos;

#[derive(Clone)]
enum Opcion {
    Profesores,
    Asignaturas,
    Salir,
}

const ITEMS_MENU_DATA: [(Opcion, menus::TextoOpcion); 3] = [
    (Opcion::Profesores, "Profesores"),
    (Opcion::Asignaturas, "Asignaturas"),
    (Opcion::Salir, "Salir"),
];

type ItemMenus<'a> = Vec<ItemMenu<'a, Opcion>>;

pub struct MenuPrincipal<'a> {
    pub consola: &'a consola::Consola,
}

impl Menu for MenuPrincipal<'_> {
    fn abrir_menu(&mut self) {
        let items_menu: ItemMenus = menus::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            match self.mostrar_iteracion_menu(&items_menu) {
                Some(SalirMenu) => {
                    break;
                }
                _ => continue,
            }
        }
    }
}

impl MenuPrincipal<'_> {
    fn mostrar_iteracion_menu(&self, items_menu: &ItemMenus) -> Option<SalirMenu> {
        self.mostrar_texto_menu(items_menu);
        let eleccion = self.consola.get_input();
        let opcion = menus::extraer_opcion(eleccion, &items_menu)?;
        match opcion {
            Opcion::Asignaturas => self.abrir_menu_asignaturas(),
            Opcion::Profesores => self.abrir_menu_profes(),
            Opcion::Salir => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_texto_menu(&self, items_menu: &ItemMenus) {
        self.consola.clear_screen();
        self.consola.mostrar_titulo(textos::MENU_PRINCIPAL);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        self.consola.mostrar(&texto_opciones);
    }

    fn abrir_menu_profes(&self) {
        let mut menu = menu_profes::MenuProfesores::new(self.consola);
        menu.abrir_menu();
    }

    fn abrir_menu_asignaturas(&self) {
        let asignaturas = repo::get_asignaturas();
        let mut menu = menu_asignaturas::MenuAsignaturas::new(self.consola, asignaturas);
        menu.abrir_menu();
    }
}

use super::menu_asignaturas;
use super::menu_profes;
use super::shared as menus;
use super::shared::Menu;
use super::shared::{ItemMenu, SalirMenu};
use crate::repo;
use crate::textos;
use crate::vista;

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
    pub vista: &'a vista::Vista,
}

impl Menu for MenuPrincipal<'_> {
    fn abrir_menu(&mut self) {
        let items_menu: ItemMenus = menus::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            if let Some(_instruccion) = self.mostrar_iteracion_menu(&items_menu) {
                break;
            }
        }
    }
}

impl MenuPrincipal<'_> {
    fn mostrar_iteracion_menu(&self, items_menu: &ItemMenus) -> Option<SalirMenu> {
        self.vista.clear_screen();
        self.vista.mostrar_titulo(textos::MENU_PRINCIPAL);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        self.vista.mostrar(&texto_opciones);

        let eleccion = self.vista.get_input();
        let opcion = menus::extraer_opcion(eleccion, &items_menu)?;
        match opcion {
            Opcion::Asignaturas => self.abrir_menu_asignaturas(),
            Opcion::Profesores => self.abrir_menu_profes(),
            Opcion::Salir => return Some(SalirMenu),
        }
        return None;
    }

    fn abrir_menu_profes(&self) {
        let mut menu = menu_profes::MenuProfesores::new(self.vista);
        menu.abrir_menu();
    }

    fn abrir_menu_asignaturas(&self) {
        let asignaturas = repo::get_asignaturas();
        let mut menu = menu_asignaturas::MenuAsignaturas::new(self.vista, asignaturas);
        menu.abrir_menu();
    }
}

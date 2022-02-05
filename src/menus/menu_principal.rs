use super::menu_asignaturas;
use super::menu_profesores;
use super::shared as menus;
use super::shared::Menu;
use super::shared::{ItemMenu, SalirMenu};
use crate::components::Control;
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

pub struct MenuPrincipal {}

impl Menu for MenuPrincipal {
    fn abrir_menu(&mut self, control: &mut Control) {
        let items_menu: ItemMenus = menus::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            match self.mostrar_iteracion_menu(&items_menu, control) {
                Some(SalirMenu) => {
                    break;
                }
                _ => continue,
            }
        }
    }
}

impl MenuPrincipal {
    fn mostrar_iteracion_menu(
        &self,
        items_menu: &ItemMenus,
        control: &mut Control,
    ) -> Option<SalirMenu> {
        self.mostrar_texto_menu(items_menu, control);
        let eleccion = control.consola.get_input();
        let opcion = menus::extraer_opcion(eleccion, &items_menu)?;
        match opcion {
            Opcion::Asignaturas => self.abrir_menu_asignaturas(control),
            Opcion::Profesores => self.abrir_menu_profesores(control),
            Opcion::Salir => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_texto_menu(&self, items_menu: &ItemMenus, control: &mut Control) {
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::MENU_PRINCIPAL);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        control.consola.mostrar(&texto_opciones);
    }

    fn abrir_menu_profesores(&self, control: &mut Control) {
        let mut menu = menu_profesores::MenuProfesores::new();
        menu.abrir_menu(control);
    }

    fn abrir_menu_asignaturas(&self, control: &mut Control) {
        let asignaturas = control
            .application
            .repository
            .persistencia
            .get_asignaturas();
        let mut menu = menu_asignaturas::MenuAsignaturas::new(asignaturas);
        menu.abrir_menu(control);
    }
}

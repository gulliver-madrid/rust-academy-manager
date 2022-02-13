use super::counter::Counter;
use super::menu_asignaturas::MenuAsignaturas;
use super::menu_profesores::MenuProfesores;
use super::shared as menus;
use super::shared::{ItemMenu, SalirMenu};
use crate::components::Control;
use crate::textos;

const LOOP_LIMIT: u32 = 200;

#[derive(Debug, Clone, PartialEq)]
pub enum Opcion {
    Profesores,
    Asignaturas,
    Salir,
}

pub const ITEMS_MENU_DATA: [(Opcion, menus::TextoOpcion); 3] = [
    (Opcion::Profesores, "Profesores"),
    (Opcion::Asignaturas, "Asignaturas"),
    (Opcion::Salir, "Salir"),
];

type ItemMenus<'a> = Vec<ItemMenu<'a, Opcion>>;

pub struct MenuPrincipal<'a> {
    pub control: &'a mut Control,
    raised_loop_limit: bool,
}

impl MenuPrincipal<'_> {
    pub fn new(control: &mut Control) -> MenuPrincipal {
        MenuPrincipal {
            control,
            raised_loop_limit: false,
        }
    }
    #[cfg(test)]
    pub fn raised_loop_limit(&self) -> bool {
        self.raised_loop_limit
    }
}

impl MenuPrincipal<'_> {
    pub fn abrir_menu(&mut self) {
        let items_menu: ItemMenus = menus::crear_items_menu(ITEMS_MENU_DATA);

        for _ in Counter::new(LOOP_LIMIT) {
            match self.mostrar_iteracion_menu(&items_menu) {
                Some(SalirMenu) => {
                    return;
                }
                _ => (),
            }
        }
        self.raised_loop_limit = true;
        println!("\nERROR: Se superó el límite de ciclos del menú principal");
    }
    fn mostrar_iteracion_menu(
        &mut self,
        items_menu: &ItemMenus,
    ) -> Option<SalirMenu> {
        self.mostrar_texto_menu(items_menu);
        let opcion = self.control.ui.get_user_choice(&items_menu)?;
        match opcion {
            Opcion::Asignaturas => self.abrir_menu_asignaturas(),
            Opcion::Profesores => self.abrir_menu_profesores(),
            Opcion::Salir => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_texto_menu(&self, items_menu: &ItemMenus) {
        self.control.ui.clear_screen();
        self.control.ui.mostrar_titulo(textos::MENU_PRINCIPAL);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        self.control.ui.mostrar(&texto_opciones);
    }

    fn abrir_menu_profesores(&mut self) {
        let mut menu = MenuProfesores::new(self.control);
        menu.abrir_menu();
    }

    fn abrir_menu_asignaturas(&mut self) {
        let mut menu = MenuAsignaturas::new(self.control);
        menu.abrir_menu();
    }
}

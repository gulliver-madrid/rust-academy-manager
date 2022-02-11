use super::menu_asignaturas::MenuAsignaturas;
use super::menu_profesores::MenuProfesores;
use super::shared as menus;
use super::shared::{ItemMenu, SalirMenu};
use crate::components::Control;
use crate::textos;

const LOOP_LIMIT: u32 = 200;

#[derive(Clone)]
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
        let mut counter = 0;
        loop {
            match self.mostrar_iteracion_menu(&items_menu) {
                Some(SalirMenu) => {
                    break;
                }
                _ => (),
            }
            counter += 1;
            if counter > LOOP_LIMIT {
                println!(
                    "\nERROR: Se superó el límite de ciclos del menú principal"
                );
                self.raised_loop_limit = true;
                break;
            }
        }
    }
    fn mostrar_iteracion_menu(
        &mut self,
        items_menu: &ItemMenus,
    ) -> Option<SalirMenu> {
        self.mostrar_texto_menu(items_menu, );
        let eleccion = self.control.consola.get_input();
        let opcion = menus::extraer_opcion(eleccion, &items_menu)?;
        match opcion {
            Opcion::Asignaturas => self.abrir_menu_asignaturas(),
            Opcion::Profesores => self.abrir_menu_profesores(),
            Opcion::Salir => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_texto_menu(&self, items_menu: &ItemMenus) {
        self.control.consola.clear_screen();
        self.control.consola.mostrar_titulo(textos::MENU_PRINCIPAL);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        self.control.consola.mostrar(&texto_opciones);
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

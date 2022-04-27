use super::menu_anadir_profesor;
use super::menu_eliminar_profesor::MenuEliminarProfesor;

use crate::components::Control;
use crate::menus::shared::{ItemMenu, SalirMenu, TextoOpcion};
use crate::menus::{shared, Menu};
use crate::textos;
use crate::views::View;

#[derive(Clone)]
enum Opcion {
    MostrarLista,
    AnadirProfesor,
    EliminarProfesor,
    Volver,
}
type ItemMenus<'a> = Vec<ItemMenu<'a, Opcion>>;

const ITEMS_MENU_DATA: [(Opcion, TextoOpcion); 4] = [
    (Opcion::MostrarLista, "Ver la lista de profesores"),
    (Opcion::AnadirProfesor, "Añadir un profesor"),
    (Opcion::EliminarProfesor, "Eliminar un profesor"),
    (Opcion::Volver, "Volver al menú principal"),
];

pub struct MenuProfesores {}
impl Menu for MenuProfesores {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}
impl MenuProfesores {
    pub fn new() -> MenuProfesores {
        MenuProfesores {}
    }
    fn _abrir_menu(&mut self, control: &mut Control) {
        control.application.teachers_app.load_profesores();
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
        self.mostrar_texto_menu(items_menu, control);

        let entrada_usuario = control.consola.get_input();
        let opcion_elegida = shared::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_profesores(control),
            Opcion::AnadirProfesor => self.abrir_menu_anadir_profesor(control),
            Opcion::EliminarProfesor => self.abrir_menu_eliminar_profesor(control),
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_texto_menu(&self, items_menu: &ItemMenus, control: &mut Control) {
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::MENU_PROFESORES);
        let texto_opciones = shared::crear_texto_opciones(&items_menu);
        control.consola.mostrar(&texto_opciones);
    }
    fn mostrar_lista_profesores(&self, control: &Control) {
        let profesores = control.application.teachers_app.get_teachers();
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::LISTA_PROFESORES);
        for profesor in profesores {
            control.consola.mostrar(&profesor.crear_linea_tabla());
        }
        control.consola.pausa_enter("volver al menú de profesores");
    }

    fn abrir_menu_anadir_profesor(&self, control: &mut Control) {
        let mut menu = menu_anadir_profesor::MenuAnadirProfesor {};
        menu.abrir_menu(control);
    }

    fn abrir_menu_eliminar_profesor(&mut self, control: &mut Control) {
        let mut menu = MenuEliminarProfesor {};
        menu.abrir_menu(control);
    }
}

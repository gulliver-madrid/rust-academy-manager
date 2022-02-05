use super::menu_anadir_profe;
use super::menu_eliminar_profesor::MenuEliminarProfesor;
use super::shared as menus;
use super::shared::ItemMenu;
use super::shared::Menu;
use super::shared::SalirMenu;
use crate::components::Control;
use crate::dominio::teachers::Profesores;
use crate::textos;
use crate::views::View;

#[derive(Clone)]
enum Opcion {
    MostrarLista,
    AnadirProfe,
    EliminarProfesor,
    Volver,
}
type ItemMenus<'a> = Vec<ItemMenu<'a, Opcion>>;

const ITEMS_MENU_DATA: [(Opcion, menus::TextoOpcion); 4] = [
    (Opcion::MostrarLista, "Ver la lista de profesores"),
    (Opcion::AnadirProfe, "Añadir un profesor"),
    (Opcion::EliminarProfesor, "Eliminar un profesor"),
    (Opcion::Volver, "Volver al menú principal"),
];

pub struct MenuProfesores {}
impl Menu for MenuProfesores {
    fn abrir_menu(&mut self, control: &Control) {
        self._abrir_menu(control);
    }
}
impl MenuProfesores {
    pub fn new() -> MenuProfesores {
        MenuProfesores {}
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
        let profesores = &mut control.repository.persistencia.load_profesores();

        let entrada_usuario = control.consola.get_input();
        let opcion_elegida = menus::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_profes(profesores, control),
            Opcion::AnadirProfe => self.abrir_menu_anadir_profe(profesores, control),
            Opcion::EliminarProfesor => {
                self.abrir_menu_eliminar_profesor(profesores, control)
            }
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_texto_menu(&self, items_menu: &ItemMenus, control: &Control) {
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::MENU_PROFESORES);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        control.consola.mostrar(&texto_opciones);
    }
    fn mostrar_lista_profes(&self, profesores: &Profesores, control: &Control) {
        control.consola.clear_screen();
        control.consola.mostrar_titulo(textos::LISTA_PROFESORES);
        for profe in profesores {
            control.consola.mostrar(&profe.crear_linea_tabla());
        }
        control.consola.pausa_enter("volver al menú de profesores");
    }

    fn abrir_menu_anadir_profe(&self, profesores: &mut Profesores, control: &Control) {
        let mut menu = menu_anadir_profe::MenuAnadirProfesor::new(profesores);
        menu.abrir_menu(control);
    }

    fn abrir_menu_eliminar_profesor(
        &mut self,
        profesores: &mut Profesores,
        control: &Control,
    ) {
        let mut menu = MenuEliminarProfesor::new(profesores);
        menu.abrir_menu(control);
    }
}

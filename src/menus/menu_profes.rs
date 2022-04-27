use super::shared as menus;
use super::shared::ItemMenu;
use super::shared::Menu;
use super::shared::SalirMenu;
use crate::helpers;
use crate::repo;
use crate::teachers::{Profesor, Profesores};
use crate::textos;
use crate::views::View;
use crate::vista;

#[derive(Clone)]
enum Opcion {
    MostrarLista,
    AnadirProfe,
    Volver,
}
type ItemMenus<'a> = Vec<ItemMenu<'a, Opcion>>;

const ITEMS_MENU_DATA: [(Opcion, menus::TextoOpcion); 3] = [
    (Opcion::MostrarLista, "Ver la lista de profesores"),
    (Opcion::AnadirProfe, "Añadir un profesor"),
    (Opcion::Volver, "Volver al menú principal"),
];

pub struct MenuProfesores<'a> {
    pub vista: &'a vista::Vista,
}
impl Menu for MenuProfesores<'_> {
    fn abrir_menu(&self) {
        let mut profesores = repo::load_profesores();
        let items_menu = menus::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            if let Some(_instruccion) =
                self.mostrar_iteracion_menu(&items_menu, &mut profesores)
            {
                break;
            }
        }
    }
}
impl MenuProfesores<'_> {
    fn mostrar_iteracion_menu(
        &self,
        items_menu: &ItemMenus,
        profesores: &mut Profesores,
    ) -> Option<SalirMenu> {
        self.vista.clear_screen();
        self.vista.mostrar(textos::TITULO_MENU_PROFESORES);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        self.vista.mostrar(&texto_opciones);
        let entrada_usuario = self.vista.get_input();
        let opcion_elegida = menus::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_profes(profesores),
            Opcion::AnadirProfe => self.abrir_menu_anadir_profe(profesores),
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_lista_profes(&self, profesores: &Profesores) {
        self.vista.clear_screen();
        self.vista.mostrar("\nLista de profesores");
        self.vista.mostrar("-------------------\n");
        for profe in profesores {
            self.vista.mostrar(&profe.crear_linea_tabla());
        }
        self.vista
            .mostrar("\nPulsa ENTER para volver al menú de profesores");
        self.vista.get_input();
    }

    fn abrir_menu_anadir_profe(&self, profesores: &mut Profesores) {
        let new_id: u32;
        {
            let last_profe = helpers::get_last_element(profesores);
            match last_profe {
                None => {
                    self.vista.mostrar("Error: no se encontró ningún profesor");
                    return;
                }
                Some(last_profe) => new_id = last_profe.id + 1,
            }
        }

        self.vista.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
        let nombre_introducido = self.vista.get_input();
        match nombre_introducido.as_str() {
            "" => return,
            _ => {
                let nuevo_profe = Profesor {
                    nombre: String::from(nombre_introducido),
                    id: new_id,
                    telefono: String::new(),
                };
                profesores.push(nuevo_profe);
                repo::save_profesores(profesores.clone());
            }
        }
    }
}

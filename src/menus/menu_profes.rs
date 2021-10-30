use super::shared as menus;
use super::shared::ItemMenu;
use super::shared::Menu;
use super::shared::SalirMenu;
use crate::consola;
use crate::dominio::teachers::Profesores;
use crate::dominio::Profesor;
use crate::helpers;
use crate::repo;
use crate::textos;
use crate::views::View;

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
    consola: &'a consola::Consola,
}
impl Menu for MenuProfesores<'_> {
    fn abrir_menu(&mut self) {
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
    pub fn new(consola: &consola::Consola) -> MenuProfesores {
        MenuProfesores { consola }
    }
    fn mostrar_iteracion_menu(
        &self,
        items_menu: &ItemMenus,
        profesores: &mut Profesores,
    ) -> Option<SalirMenu> {
        self.consola.clear_screen();
        self.consola.mostrar_titulo(textos::MENU_PROFESORES);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        self.consola.mostrar(&texto_opciones);
        let entrada_usuario = self.consola.get_input();
        let opcion_elegida = menus::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_profes(profesores),
            Opcion::AnadirProfe => self.abrir_menu_anadir_profe(profesores),
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }
    fn mostrar_lista_profes(&self, profesores: &Profesores) {
        self.consola.clear_screen();
        self.consola.mostrar_titulo(textos::LISTA_PROFESORES);
        for profe in profesores {
            self.consola.mostrar(&profe.crear_linea_tabla());
        }
        self.consola
            .mostrar("\nPulsa ENTER para volver al menú de profesores");
        self.consola.get_input();
    }

    fn abrir_menu_anadir_profe(&self, profesores: &mut Profesores) {
        let new_id: u32;
        {
            let last_profe = helpers::get_last_element(profesores);
            match last_profe {
                None => {
                    self.consola
                        .mostrar("Error: no se encontró ningún profesor");
                    return;
                }
                Some(last_profe) => new_id = last_profe.id + 1,
            }
        }

        self.consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
        let nombre_introducido = self.consola.get_input();
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

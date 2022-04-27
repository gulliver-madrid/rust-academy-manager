use super::shared::Menu;
use super::shared::{ItemMenu, SalirMenu};
use crate::asignatura::Asignatura;
use crate::asignatura::Asignaturas;
use crate::helpers;
use crate::repo;
use crate::textos;
use crate::views::View;
use crate::vista;

use super::shared as menus;

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
    pub vista: &'a vista::Vista,
}

impl Menu for MenuAsignaturas<'_> {
    fn abrir_menu(&self) {
        let mut asignaturas = repo::get_asignaturas();
        let items_menu = menus::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            if let Some(_instruccion) =
                self.mostrar_iteracion_menu(&items_menu, &mut asignaturas)
            {
                break;
            }
        }
    }
}

impl MenuAsignaturas<'_> {
    fn mostrar_iteracion_menu(
        &self,
        items_menu: &ItemMenus,
        asignaturas: &mut Asignaturas,
    ) -> Option<SalirMenu> {
        self.vista.clear_screen();
        self.vista.mostrar(textos::TITULO_MENU_ASIGNATURAS);
        let texto_opciones = menus::crear_texto_opciones(&items_menu);
        self.vista.mostrar(&texto_opciones);

        let entrada_usuario = self.vista.get_input();
        let opcion_elegida = menus::extraer_opcion(entrada_usuario, &items_menu)?;
        match opcion_elegida {
            Opcion::MostrarLista => self.mostrar_lista_asignaturas(asignaturas),
            Opcion::AnadirAsignatura => self.abrir_menu_anadir_asignatura(asignaturas),
            Opcion::Volver => return Some(SalirMenu),
        }
        return None;
    }

    fn mostrar_lista_asignaturas(&self, asignaturas: &Asignaturas) {
        self.vista.clear_screen();
        self.vista.mostrar_titulo("Lista de asignaturas");
        let texto_lista_asignaturas = self.crear_lista_asignaturas(asignaturas);
        self.vista.mostrar(texto_lista_asignaturas.as_str());
        self.vista
            .mostrar("\nPulsa ENTER para volver al menú de asignaturas");
        self.vista.get_input();
    }

    fn crear_lista_asignaturas(&self, asignaturas: &Asignaturas) -> String {
        asignaturas
            .iter()
            .map(|asignatura| asignatura.crear_linea_tabla())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn abrir_menu_anadir_asignatura(&self, asignaturas: &mut Asignaturas) {
        let new_id: u32;
        {
            let last_profe = helpers::get_last_element(asignaturas);
            match last_profe {
                None => {
                    self.vista.mostrar("Error: no se encontró ningún profesor");
                    return;
                }
                Some(last_profe) => new_id = last_profe.id + 1,
            }
        }

        self.vista.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA);
        let nombre_introducido = self.vista.get_input();
        match nombre_introducido.as_str() {
            "" => return,
            _ => {
                let nueva = Asignatura {
                    nombre: String::from(nombre_introducido),
                    id: new_id,
                };
                asignaturas.push(nueva);
                repo::save_asignaturas(asignaturas.clone());
            }
        }
    }
}

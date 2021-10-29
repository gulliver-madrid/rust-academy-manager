use crate::menu_asignaturas;
use crate::menu_profes;
use crate::shared_menus as menus;
use crate::shared_menus::ItemMenu;
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

struct Salir;

pub struct MenuPrincipal<'a> {
    pub vista: &'a vista::Vista,
}

impl<'a> MenuPrincipal<'a> {
    pub fn abrir_menu(&self) {
        let items_menu: ItemMenus = menus::crear_items_menu(ITEMS_MENU_DATA);
        loop {
            if let Some(_instruccion) = self.mostrar_iteracion_menu(&items_menu) {
                break;
            }
        }
    }

    fn mostrar_iteracion_menu(&self, items_menu: &ItemMenus) -> Option<Salir> {
        self.vista.clear_screen();
        self.vista.mostrar(textos::TITULO_MENU_PRINCIPAL);
        let opciones_string = menus::crear_texto_opciones(&items_menu);
        self.vista.mostrar(&opciones_string);

        let eleccion = self.vista.get_input();
        let opcion = menus::extraer_opcion(eleccion, &items_menu)?;
        match opcion {
            Opcion::Asignaturas => self.abrir_menu_asignaturas(),
            Opcion::Profesores => self.abrir_menu_profes(),
            Opcion::Salir => return Some(Salir),
        }
        return None;
    }

    fn abrir_menu_profes(&self) {
        let menu = menu_profes::MenuProfesores {};
        menu.abrir_menu(self.vista);
    }

    fn abrir_menu_asignaturas(&self) {
        let menu = menu_asignaturas::MenuAsignaturas {};
        menu.abrir_menu(self.vista);
    }
}

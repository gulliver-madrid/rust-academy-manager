use crate::{
    consola::InnerConsole,
    dominio::{Asignatura, Asignaturas, Profesor, Profesores},
    menus::ITEMS_MENU_DATA,
    repository::PersistenciaTrait,
};

#[cfg(test)] // El compilador solo lo detecta en modo test
use crate::{
    application::Application,
    components::Control,
    consola,
    menus::{Menu, MenuPrincipal},
};

pub struct MockConsole {}

impl InnerConsole for MockConsole {
    fn clear_screen(&self) {}
    fn get_input(&self) -> String {
        let num_opciones = ITEMS_MENU_DATA.len();
        let ultima_opcion = num_opciones.to_string();
        ultima_opcion
    }
    fn mostrar(&self, _texto: &str) {}
}

pub struct MockPersistencia {}
impl PersistenciaTrait for MockPersistencia {
    fn save_profesores(&self, _profesores: &Profesores) {}
    fn save_asignaturas(&self, _asignaturas: &Asignaturas) {}
    fn load_profesores(&self) -> Profesores {
        return Vec::<Profesor>::new();
    }
    fn load_subjects(&self) -> Asignaturas {
        return Vec::<Asignatura>::new();
    }
}

#[test]
fn salir_desde_menu_principal() {
    let persistencia = MockPersistencia {};
    let application = Application::new(Box::new(persistencia));
    let consola = consola::Consola {
        inner_console: Box::new(MockConsole {}),
    };
    let mut control = Control {
        consola,
        application,
    };
    let mut menu = MenuPrincipal::new();
    menu.abrir_menu(&mut control);
    assert_eq!(menu.raised_loop_limit(), false);
}

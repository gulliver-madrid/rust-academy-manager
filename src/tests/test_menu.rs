#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)] // El compilador solo lo detecta en modo test
use crate::{
    application::Application,
    components::Control,
    consola,
    consola::InnerConsole,
    dominio::{Asignatura, Asignaturas, Profesor, Profesores},
    menus::MenuPrincipal,
    menus::ITEMS_MENU_DATA,
    repository::PersistenciaTrait,
};

#[cfg(test)]
pub struct MockConsole {
    pub provided_inputs: RefCell<Vec<String>>,
}

#[cfg(test)]
impl MockConsole {
    fn new() -> Self {
        Self {
            provided_inputs: RefCell::new(Vec::<String>::new()),
        }
    }
}
#[cfg(test)]
impl InnerConsole for MockConsole {
    fn clear_screen(&self) {}
    fn get_input(&self) -> String {
        self.provided_inputs.borrow_mut().pop().unwrap()
    }
    fn mostrar(&self, _texto: &str) {}
}

#[cfg(test)]
pub struct MockPersistencia {
    pub mock_profesores: Vec<Profesor>,
    pub mock_asignaturas: Vec<Asignatura>,
}

#[cfg(test)]
impl PersistenciaTrait for MockPersistencia {
    fn save_profesores(&self, _profesores: &Profesores) {}
    fn save_asignaturas(&self, _asignaturas: &Asignaturas) {}
    fn load_profesores(&self) -> Profesores {
        return self.mock_profesores.clone();
    }
    fn load_subjects(&self) -> Asignaturas {
        return self.mock_asignaturas.clone();
    }
}
#[cfg(test)]
fn get_last_option_main_menu() -> String {
    let num_opciones = ITEMS_MENU_DATA.len();
    let ultima_opcion = num_opciones.to_string();
    ultima_opcion
}
#[test]
fn salir_desde_menu_principal() {
    let persistencia = MockPersistencia {
        mock_profesores: Vec::<Profesor>::new(),
        mock_asignaturas: Vec::<Asignatura>::new(),
    };
    let application = Application::new(Box::new(persistencia));
    let provided_input = get_last_option_main_menu();
    let mock_console = MockConsole::new();
    mock_console
        .provided_inputs
        .borrow_mut()
        .push(provided_input);
    let consola = consola::Consola {
        inner_console: Box::new(mock_console),
    };
    let mut control = Control {
        consola,
        application,
    };
    let mut menu = MenuPrincipal::new(&mut control);
    menu.abrir_menu();
    assert_eq!(menu.raised_loop_limit(), false);
}

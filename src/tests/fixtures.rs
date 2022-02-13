#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
use crate::{
    application::Application,
    components::Control,
    dominio::{Asignatura, Asignaturas, Profesor, Profesores},
    menus::shared::TextoOpcion,
    repository::PersistenciaTrait,
    tests::mock_console::MockConsole,
    ui::UserInterface,
};

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
pub fn choice_to_string<'a, OpcionMenu: PartialEq + Debug, const N: usize>(
    opcion: OpcionMenu,
    items_menu_data: [(OpcionMenu, TextoOpcion); N],
) -> Option<String> {
    let mut index = 1;
    for tupla in items_menu_data {
        if tupla.0 == opcion {
            return Some(index.to_string());
        }
        index += 1;
    }
    None
}

#[cfg(test)]
pub fn crear_application_con_persistencia_vacia() -> Application {
    let persistencia = MockPersistencia {
        mock_profesores: Vec::<Profesor>::new(),
        mock_asignaturas: Vec::<Asignatura>::new(),
    };
    Application::new(Box::new(persistencia))
}

#[cfg(test)]
pub fn crear_control(
    mock_console: MockConsole,
    application: Application,
) -> Control {
    let ui = UserInterface {
        inner_console: Box::new(mock_console),
    };
    Control { ui, application }
}

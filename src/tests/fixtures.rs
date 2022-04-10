#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
use crate::{
    application::Application,
    components::Control,
    domain::{Subject, Subjects, Teacher, Teachers},
    menus::shared::OptionText,
    repository::PersistenceTrait,
    tests::mock_console::MockConsole,
    ui::UserInterface,
};

#[cfg(test)]
pub struct MockPersistencia {
    pub mock_profesores: Vec<Teacher>,
    pub mock_asignaturas: Vec<Subject>,
}

#[cfg(test)]
impl PersistenceTrait for MockPersistencia {
    fn save_teachers(&self, _profesores: &Teachers) {}
    fn save_subjects(&self, _asignaturas: &Subjects) {}
    fn load_teachers(&self) -> Teachers {
        return self.mock_profesores.clone();
    }
    fn load_subjects(&self) -> Subjects {
        return self.mock_asignaturas.clone();
    }
}

#[cfg(test)]
pub fn choice_to_string<'a, MenuOption: PartialEq + Debug, const N: usize>(
    opcion: MenuOption,
    items_menu_data: [(MenuOption, OptionText); N],
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
pub fn create_application_with_void_persistence() -> Application {
    let persistencia = MockPersistencia {
        mock_profesores: Vec::<Teacher>::new(),
        mock_asignaturas: Vec::<Subject>::new(),
    };
    Application::new(Box::new(persistencia))
}

#[cfg(test)]
pub fn create_control(
    mock_console: MockConsole,
    application: Application,
) -> Control {
    let ui = UserInterface {
        inner_console: Box::new(mock_console),
    };
    Control { ui, application }
}

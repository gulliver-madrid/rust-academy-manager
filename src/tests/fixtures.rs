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
pub struct MockPersistence {
    pub mock_teachers: Vec<Teacher>,
    pub mock_subjects: Vec<Subject>,
}

#[cfg(test)]
impl PersistenceTrait for MockPersistence {
    fn save_teachers(&self, _teachers: &Teachers) {}
    fn save_subjects(&self, _subjects: &Subjects) {}
    fn load_teachers(&self) -> Teachers {
        return self.mock_teachers.clone();
    }
    fn load_subjects(&self) -> Subjects {
        return self.mock_subjects.clone();
    }
}

#[cfg(test)]
/// Returns the option number (indexing from 1)
pub fn choice_to_string<'a, MenuOption: PartialEq + Debug, const N: usize>(
    option: MenuOption,
    items_menu_data: [(MenuOption, OptionText); N],
) -> Option<String> {
    let mut index = 1;
    for item in items_menu_data {
        if item.0 == option {
            return Some(index.to_string());
        }
        index += 1;
    }
    None
}

#[cfg(test)]
pub fn create_application_with_void_persistence() -> Application {
    use crate::application::create_application;

    let persistence = MockPersistence {
        mock_teachers: Vec::<Teacher>::new(),
        mock_subjects: Vec::<Subject>::new(),
    };
    create_application(Box::new(persistence))
}

#[cfg(test)]
use std::rc::Rc;
#[cfg(test)]
pub fn create_control(
    mock_console: Rc<MockConsole>,
    application: Application,
) -> Control {
    let ui = UserInterface {
        inner_console: mock_console,
    };
    Control { ui, application }
}

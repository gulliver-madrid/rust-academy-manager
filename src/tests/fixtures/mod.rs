#![cfg(test)]

use std::rc::Rc;

use crate::{
    application::Application, // fmt
    components::Control,
    tests::mocks::{mock_console::MockConsole, mock_persistence},
    ui::UserInterface,
};

pub fn create_application_with_void_persistence() -> Application {
    use crate::application::create_application;
    let persistence = mock_persistence::create_void_mock_persistence();
    create_application(Rc::new(persistence))
}

pub fn create_control(
    mock_console: Rc<MockConsole>,
    application: Application,
) -> Control {
    let ui = UserInterface {
        inner_console: mock_console,
    };
    Control { ui, application }
}

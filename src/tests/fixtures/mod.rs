#![cfg(test)]

use std::rc::Rc;

use crate::{
    application::Application, // fmt
    components::Control,
    tests::mocks::{mock_console::MockConsole, mock_persistence},
    ui::UserInterface,
};

use super::mocks::mock_persistence::MockPersistence;

pub fn create_application_with_mock_persistence(
    mock_persistence: Option<MockPersistence>,
) -> Application {
    use crate::application::create_application;
    let persistence: MockPersistence = match mock_persistence {
        None => mock_persistence::create_void_mock_persistence(),
        Some(mock_persistence) => mock_persistence,
    };
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

#![cfg(test)]

pub mod mock_persistence;

use colored::*;
use std::fmt::Debug;

use crate::{
    application::Application, // fmt
    components::Control,
    menus::shared::OptionText,
    tests::mock_console::MockConsole,
    ui::UserInterface,
};

/// Returns the option number (indexing from 1)
pub fn choice_to_string<'a, MenuOption: PartialEq + Debug>(
    option: MenuOption,
    items_menu_data: &[(MenuOption, OptionText)],
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

pub fn create_application_with_void_persistence() -> Application {
    use crate::application::create_application;
    let persistence = mock_persistence::create_void_mock_persistence();
    create_application(Rc::new(persistence))
}

use std::rc::Rc;

pub fn create_control(
    mock_console: Rc<MockConsole>,
    application: Application,
) -> Control {
    let ui = UserInterface {
        inner_console: mock_console,
    };
    Control { ui, application }
}

/// Utility for improving test fail messages
pub fn highlight(s: &str) -> String {
    println!("En highlight hemos recibido {}", s);
    format!("\n\n{}: {}\n\n", "FAIL".red(), s.truecolor(255, 180, 0))
}

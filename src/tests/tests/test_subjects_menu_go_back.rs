#![cfg(test)]

use pretty_assertions::assert_eq;
use std::rc::Rc;

use crate::{
    menus::{
        MainMenu, // fmt
        MainMenuOption,
        SubjectsMenuOption,
        ITEMS_MENU_DATA__MAIN_MENU,
        ITEMS_MENU_DATA__SUBJECTS_MENU,
    },
    tests::{
        fixtures::{
            create_application_with_mock_persistence, // fmt
            create_control,
        },
        helpers::choice_to_string,
        mocks::mock_console::MockConsole,
    },
};

#[test]
fn enter_to_subjects_and_exit() {
    let application = create_application_with_mock_persistence(None);
    let inputs = [
        choice_to_string(MainMenuOption::Subjects, &ITEMS_MENU_DATA__MAIN_MENU),
        choice_to_string(SubjectsMenuOption::GoBack, &ITEMS_MENU_DATA__SUBJECTS_MENU),
        choice_to_string(MainMenuOption::Exit, &ITEMS_MENU_DATA__MAIN_MENU),
    ];

    let mock_console = Rc::new(MockConsole::new());
    mock_console.add_inputs(&inputs);
    let control = create_control(Rc::clone(&mock_console), application);
    let mut menu = MainMenu::new(Rc::new(control));
    menu.open_menu();
    assert_eq!(menu.loop_limit_exceed(), false);
    mock_console.show_all();
}

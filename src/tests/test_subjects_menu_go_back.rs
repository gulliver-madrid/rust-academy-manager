#![cfg(test)]

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
            choice_to_string, // fmt
            create_application_with_void_persistence,
            create_control,
        },
        mock_console::MockConsole,
    },
};

#[test]
fn enter_to_subjects_and_exit() {
    let application = create_application_with_void_persistence();
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

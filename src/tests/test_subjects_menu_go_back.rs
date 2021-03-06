#[cfg(test)]
use crate::{
    menus::MainMenu,
    menus::MainMenuOption,
    menus::SubjectsMenuOption,
    menus::ITEMS_MENU_DATA__MAIN_MENU,
    menus::ITEMS_MENU_DATA__SUBJECTS_MENU,
    tests::{
        fixtures::{
            choice_to_string, create_application_with_void_persistence,
            create_control,
        },
        mock_console::MockConsole,
    },
};

#[test]
fn enter_to_subjects_and_exit() {
    let application = create_application_with_void_persistence();
    let inputs = [
        choice_to_string(MainMenuOption::Subjects, ITEMS_MENU_DATA__MAIN_MENU),
        choice_to_string(SubjectsMenuOption::GoBack, ITEMS_MENU_DATA__SUBJECTS_MENU),
        choice_to_string(MainMenuOption::Exit, ITEMS_MENU_DATA__MAIN_MENU),
    ];

    let mock_console = MockConsole::new();
    {
        let mut provided_inputs = mock_console.provided_inputs.borrow_mut();
        for input in inputs {
            provided_inputs.push(input.unwrap());
        }
    }

    let mut control = create_control(mock_console, application);
    let mut menu = MainMenu::new(&mut control);
    menu.open_menu();
    assert_eq!(menu.loop_limit_exceed(), false);
}

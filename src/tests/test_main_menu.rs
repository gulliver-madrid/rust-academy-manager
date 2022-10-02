#![cfg(test)]

use std::rc::Rc;

use crate::{
    menus::{
        MainMenu, // fmt
        MainMenuOption,
        ITEMS_MENU_DATA__MAIN_MENU,
    },
    tests::{
        fixtures::{
            create_application_with_void_persistence, // fmt
            create_control,
        },
        helpers::choice_to_string,
        mocks::mock_console::MockConsole,
    },
};

#[test]
fn salir_desde_menu_principal() {
    let application = create_application_with_void_persistence();
    let provided_input =
        choice_to_string(MainMenuOption::Exit, &ITEMS_MENU_DATA__MAIN_MENU).unwrap();
    let mock_console = Rc::new(MockConsole::new());
    mock_console.add_input(provided_input);
    let control = create_control(Rc::clone(&mock_console), application);
    let mut menu = MainMenu::new(Rc::new(control));
    menu.open_menu();
    assert_eq!(menu.loop_limit_exceed(), false);
}

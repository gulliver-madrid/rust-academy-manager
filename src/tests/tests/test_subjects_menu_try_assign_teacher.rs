#![cfg(test)]

use pretty_assertions::assert_eq;
use rust_i18n::t;
use std::rc::Rc;

use crate::{
    domain::Subject,
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
        helpers::{choice_to_string, highlight},
        mocks::{
            mock_console::MockConsole, mock_persistence::create_void_mock_persistence,
        },
    },
};
/// Opens the subjects menu, try to assign a teacher to a subject, but types a no-numeric id and answer 'yes' when prompted to exit
#[test]
fn subjects_menu_try_assign_teacher() {
    const SUBJECT_NAME: &str = "Alemán";
    let expected_error_msg = t!("teacher_id_should_be_a_number");
    let mock_persistence = create_void_mock_persistence();
    mock_persistence.mock_subjects.borrow_mut().push(Subject {
        name: SUBJECT_NAME.to_string(),
        id: 777,
        assigned_teachers: Vec::new(),
    });
    let application = create_application_with_mock_persistence(Some(mock_persistence));
    // application.subjects_app.borrow().load_subjects_if_needed();
    let inputs = [
        choice_to_string(MainMenuOption::Subjects, &ITEMS_MENU_DATA__MAIN_MENU),
        choice_to_string(
            SubjectsMenuOption::AssignTeacher,
            &ITEMS_MENU_DATA__SUBJECTS_MENU,
        ),
        SUBJECT_NAME.to_string(),
        "invalid-id".to_string(),
        "y".to_string(),
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
    assert!(
        mock_console.outputs.borrow().contains(&expected_error_msg),
        "{}\n{:#?}",
        highlight("Outputs should include a no_valid_id msg"),
        mock_console.outputs.borrow()
    );
}

use rust_i18n::t;

use super::remove_subject_menu::RemoveSubjectMenu;

use crate::components::Control;

use crate::domain::Subjects;
use crate::menus::assign_teacher_menu::AssignTeacherMenu;
use crate::menus::shared::{self, MenuExit, MenuItem};
use crate::views::View;

use super::add_subject_menu::AddSubjectMenu;

#[derive(Debug, Clone, PartialEq)]
pub enum MenuOption {
    ShowList,
    AddSubject,
    RemoveSubject,
    AssignTeacher,
    GoBack,
}

type MenuItems<'a> = Vec<MenuItem<'a, MenuOption>>;

pub const MENU_ITEMS_DATA: [(MenuOption, shared::OptionText); 5] = [
    (MenuOption::ShowList, "subjects_menu_options.show_list"),
    (MenuOption::AddSubject, "subjects_menu_options.add_subject"),
    (
        MenuOption::RemoveSubject,
        "subjects_menu_options.remove_subject",
    ),
    (
        MenuOption::AssignTeacher,
        "subjects_menu_options.assign_teacher",
    ),
    (MenuOption::GoBack, "subjects_menu_options.go_back"),
];

pub struct SubjectsMenu<'a> {
    pub control: &'a mut Control,
}

impl SubjectsMenu<'_> {
    pub fn new(control: &mut Control) -> SubjectsMenu {
        SubjectsMenu { control }
    }
    pub fn open_menu(&mut self) {
        self.control
            .application
            .subjects_app
            .load_subjects_if_needed();
        let menu_items = shared::create_menu_items(MENU_ITEMS_DATA);
        loop {
            match self.show_iteration_menu(&menu_items) {
                Some(MenuExit) => {
                    break;
                }
                _ => continue,
            }
        }
    }

    fn show_iteration_menu(&mut self, menu_items: &MenuItems) -> Option<MenuExit> {
        self.show_menu_text(menu_items);
        let chosen_option = self.control.ui.get_user_choice(&menu_items)?;
        match chosen_option {
            MenuOption::ShowList => self.show_subjects_list(),
            MenuOption::AddSubject => self.open_add_subject_menu(),
            MenuOption::RemoveSubject => self.open_remove_subject_menu(),
            MenuOption::AssignTeacher => self.open_assign_teacher_to_subject_menu(),
            MenuOption::GoBack => return Some(MenuExit),
        }
        return None;
    }

    fn show_menu_text(&self, menu_items: &MenuItems) {
        let ui = &self.control.ui;
        ui.clear_screen();
        ui.show_title(t!("subjects_menu"));
        let options_text = shared::create_options_text(menu_items);
        ui.show(options_text);
    }

    fn show_subjects_list(&self) {
        let ui = &self.control.ui;
        let subjects = self.control.application.subjects_app.get_subjects();
        match subjects {
            Ok(subjects) => {
                let subjects_list_text = self.create_subjects_list(subjects);
                ui.clear_screen();
                ui.show_title(t!("subjects_list"));
                ui.show(subjects_list_text);
            }
            Err(e) => ui.show(e.to_string()),
        }
        ui.pause_enter(&t!("back_subjects_menu"));
    }

    fn create_subjects_list(&self, subjects: Subjects) -> String {
        subjects
            .iter()
            .map(|subject| subject.create_table_row())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn open_add_subject_menu(&mut self) {
        AddSubjectMenu {
            control: self.control,
        }
        .open_menu();
    }
    fn open_remove_subject_menu(&mut self) {
        RemoveSubjectMenu {
            control: self.control,
        }
        .open_menu();
    }
    fn open_assign_teacher_to_subject_menu(&mut self) {
        AssignTeacherMenu {
            control: self.control,
        }
        .open_menu();
    }
}

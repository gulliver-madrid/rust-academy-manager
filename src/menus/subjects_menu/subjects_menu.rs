use std::rc::Rc;

use rust_i18n::t;

use super::remove_subject_menu::RemoveSubjectMenu;

use crate::components::Control;

use crate::create_menu_options;
use crate::domain::Subject;
use crate::menus::assign_teacher_menu::AssignTeacherMenu;
use crate::menus::shared::{create_menu_items, create_options_text, MenuExit};
use crate::views::View;

use super::add_subject_menu::AddSubjectMenu;

create_menu_options!(
    (ShowList, "subjects_menu_options.show_list"),
    (AddSubject, "subjects_menu_options.add_subject"),
    (RemoveSubject, "subjects_menu_options.remove_subject"),
    (AssignTeacher, "subjects_menu_options.assign_teacher"),
    (GoBack, "subjects_menu_options.go_back")
);

pub struct SubjectsMenu {
    control: Rc<Control>,
}

impl SubjectsMenu {
    pub fn new(control: &Rc<Control>) -> Self {
        Self {
            control: Rc::clone(control),
        }
    }
    pub fn open_menu(&self) {
        self.control
            .application
            .subjects_app
            .borrow()
            .load_subjects_if_needed();
        let menu_items = create_menu_items(&MENU_ITEMS_DATA);
        loop {
            match self.show_iteration_menu(&menu_items) {
                Some(MenuExit) => break,
                _ => continue,
            }
        }
    }

    fn show_iteration_menu(&self, menu_items: &MenuItems) -> Option<MenuExit> {
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
        let options_text = create_options_text(menu_items);
        ui.show(options_text);
    }

    fn show_subjects_list(&self) {
        let ui = &self.control.ui;
        let subjects = self
            .control
            .application
            .subjects_app
            .borrow()
            .get_subjects_copy();
        match subjects {
            Ok(subjects) => {
                let subjects_list_text = create_subjects_list(subjects);
                ui.clear_screen();
                ui.show_title(t!("subjects_list"));
                ui.show(subjects_list_text);
            }
            Err(e) => ui.show(e.to_string()),
        }
        ui.pause_enter(&t!("back_subjects_menu"));
    }

    fn open_add_subject_menu(&self) {
        AddSubjectMenu::new(&self.control).open_menu();
    }
    fn open_remove_subject_menu(&self) {
        RemoveSubjectMenu::new(&self.control).open_menu();
    }
    fn open_assign_teacher_to_subject_menu(&self) {
        AssignTeacherMenu::new(&self.control).open_menu();
    }
}

fn create_subjects_list(subjects: Vec<Subject>) -> String {
    subjects
        .iter()
        .map(|subject| subject.create_table_row())
        .collect::<Vec<String>>()
        .join("\n")
}

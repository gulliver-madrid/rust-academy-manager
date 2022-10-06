use std::rc::Rc;

use rust_i18n::t;

use super::{add_teacher_menu::AddTeacherMenu, remove_teacher_menu::RemoveTeacherMenu};

use crate::{
    components::Control,
    create_menu_options,
    menus::shared::{self, MenuExit},
    views::View,
};

create_menu_options!(
    (ShowList, "teachers_menu_options.show_list"),
    (AddTeacher, "teachers_menu_options.add_teacher"),
    (RemoveTeacher, "teachers_menu_options.remove_teacher"),
    (GoBack, "teachers_menu_options.go_back")
);

pub struct TeachersMenu {
    control: Rc<Control>,
}

impl TeachersMenu {
    pub fn new(control: &Rc<Control>) -> Self {
        Self {
            control: Rc::clone(control),
        }
    }

    pub fn open_menu(&self) {
        self.control
            .application
            .teachers_app
            .borrow()
            .load_teachers_if_needed();
        let menu_items = shared::create_menu_items(&MENU_ITEMS_DATA);
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
            MenuOption::ShowList => self.show_teachers_list(),
            MenuOption::AddTeacher => self.open_add_teacher_menu(),
            MenuOption::RemoveTeacher => self.open_remove_teacher_menu(),
            MenuOption::GoBack => return Some(MenuExit),
        }
        return None;
    }
    fn show_menu_text(&self, menu_items: &MenuItems) {
        let ui = &self.control.ui;
        ui.clear_screen();
        ui.show_title(t!("teachers_menu"));
        let options_text = shared::create_options_text(&menu_items);
        ui.show(options_text);
    }
    fn show_teachers_list(&self) {
        let teachers = self
            .control
            .application
            .teachers_app
            .borrow()
            .get_teachers_copy();
        let ui = &self.control.ui;
        ui.clear_screen();
        ui.show_title(t!("teachers_list"));
        for teacher in teachers {
            ui.show(teacher.create_table_row());
        }
        ui.pause_enter(&t!("back_to_teachers_menu"));
    }

    fn open_add_teacher_menu(&self) {
        AddTeacherMenu::new(&self.control).open_menu();
    }

    fn open_remove_teacher_menu(&self) {
        RemoveTeacherMenu::new(&self.control).open_menu();
    }
}

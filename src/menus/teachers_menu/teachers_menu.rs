use super::add_teacher_menu;
use super::remove_teacher_menu::RemoveTeacherMenu;

use crate::components::Control;

use crate::menus::shared;
use crate::menus::shared::{MenuExit, MenuItem, OptionText};
use crate::texts;
use crate::views::View;

#[derive(Clone)]
enum MenuOption {
    ShowList,
    AddTeacher,
    RemoveTeacher,
    GoBack,
}
type MenuItems<'a> = Vec<MenuItem<'a, MenuOption>>;

const MENU_ITEMS_DATA: [(MenuOption, OptionText); 4] = [
    (MenuOption::ShowList, "Ver la lista de profesores"),
    (MenuOption::AddTeacher, "Añadir un profesor"),
    (MenuOption::RemoveTeacher, "Eliminar un profesor"),
    (MenuOption::GoBack, "Volver al menú principal"),
];

pub struct TeachersMenu<'a> {
    control: &'a mut Control,
}

impl<'a> TeachersMenu<'_> {
    pub fn new(control: &mut Control) -> TeachersMenu {
        TeachersMenu { control }
    }

    pub fn open_menu(&mut self) {
        self.control.application.teachers_app.load_teachers();
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
        ui.show_title(texts::TEACHERS_MENU);
        let options_text = shared::create_options_text(&menu_items);
        ui.show(&options_text);
    }
    fn show_teachers_list(&self) {
        let teachers = self.control.application.teachers_app.get_teachers();
        let ui = &self.control.ui;
        ui.clear_screen();
        ui.show_title(texts::TEACHERS_LIST);
        for teacher in teachers {
            ui.show(&teacher.create_table_row());
        }
        ui.pause_enter("volver al menú de profesores");
    }

    fn open_add_teacher_menu(&mut self) {
        let mut menu = add_teacher_menu::AddTeacherMenu {
            control: self.control,
        };
        menu.open_menu();
    }

    fn open_remove_teacher_menu(&mut self) {
        let mut menu = RemoveTeacherMenu {
            control: self.control,
        };
        menu.open_menu();
    }
}

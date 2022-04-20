use rust_i18n::t;

use super::counter::Counter;
use super::shared as menus;
use super::shared::{MenuExit, MenuItem};
use super::subjects_menu::SubjectsMenu;
use super::teachers_menu::TeachersMenu;
use crate::components::Control;


const LOOP_LIMIT: u32 = 200;

#[derive(Debug, Clone, PartialEq)]
pub enum MenuOption {
    Teachers,
    Subjects,
    Exit,
}

pub const MENU_ITEMS_DATA: [(MenuOption, menus::OptionText); 3] = [
    (MenuOption::Teachers, "menu_options.teachers"),
    (MenuOption::Subjects, "menu_options.subjects"),
    (MenuOption::Exit, "menu_options.exit"),
];

type MenuItems<'a> = Vec<MenuItem<'a, MenuOption>>;

/// Main menu of the academy-manager app
pub struct MainMenu<'a> {
    pub control: &'a mut Control,
    loop_limit_exceed: bool,
}

impl MainMenu<'_> {
    pub fn new(control: &mut Control) -> MainMenu {
        MainMenu {
            control,
            loop_limit_exceed: false,
        }
    }
    #[cfg(test)]
    pub fn loop_limit_exceed(&self) -> bool {
        self.loop_limit_exceed
    }
}

impl MainMenu<'_> {
    pub fn open_menu(&mut self) {
        let menu_items: MenuItems = menus::create_menu_items(MENU_ITEMS_DATA);

        for _ in Counter::new(LOOP_LIMIT) {
            match self.show_iteration_menu(&menu_items) {
                Some(MenuExit) => {
                    return;
                }
                _ => (),
            }
        }
        self.loop_limit_exceed = true;
        println!("\nERROR: Main menu loop limit was exceed");
    }
    fn show_iteration_menu(&mut self, menu_items: &MenuItems) -> Option<MenuExit> {
        self.show_menu_text(menu_items);
        let opcion = self.control.ui.get_user_choice(&menu_items)?;
        match opcion {
            MenuOption::Subjects => self.open_subjects_menu(),
            MenuOption::Teachers => self.open_teachers_menu(),
            MenuOption::Exit => return Some(MenuExit),
        }
        return None;
    }
    fn show_menu_text(&self, menu_items: &MenuItems) {
        self.control.ui.clear_screen();
        self.control.ui.show_title(&t!("main_menu"));
        let options_text = menus::create_options_text(&menu_items);
        self.control.ui.show(&options_text);
    }

    fn open_teachers_menu(&mut self) {
        let mut menu = TeachersMenu::new(self.control);
        menu.open_menu();
    }

    fn open_subjects_menu(&mut self) {
        let mut menu = SubjectsMenu::new(self.control);
        menu.open_menu();
    }
}

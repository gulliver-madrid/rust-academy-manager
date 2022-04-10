mod assign_teacher_menu;
mod counter;
mod main_menu;
pub mod shared;
mod subjects_menu;
mod teachers_menu;

pub use main_menu::MenuOption as MainMenuOption;
pub use main_menu::{MainMenu, MENU_ITEMS_DATA as ITEMS_MENU_DATA__MAIN_MENU};
pub use subjects_menu::{SubjectsMenuOption, ITEMS_MENU_DATA__SUBJECTS_MENU};

use crate::helpers;
use rust_i18n::t;

use crate::menus::shared::MenuItem;

use super::inner_console::InnerConsole;

pub struct UserInterface {
    pub inner_console: Box<dyn InnerConsole>,
}

impl UserInterface {
    pub fn get_user_choice<'a, T>(
        &self,
        menu_items: &'a Vec<MenuItem<T>>,
    ) -> Option<&'a T> {
        // T is a enum that represents a menu option
        let choice = self.get_input();
        crate::menus::shared::extract_option(choice, &menu_items)
    }

    pub fn clear_screen(&self) {
        self.inner_console.clear_screen();
    }

    pub fn get_input(&self) -> String {
        self.inner_console.get_input()
    }

    pub fn ask_text_to_user(&self) -> Option<String> {
        match self.get_input().trim() {
            "" => None,
            text => Some(String::from(text)),
        }
    }

    pub fn show(&self, text: &str) {
        self.inner_console.show(text);
    }

    pub fn show_title(&self, text: &str) {
        self.show(&self.convert_to_title(text));
    }

    pub fn pause_enter(&self, text: &str) {
        self.show(&format!("{} {}", t!("press_enter_to"), text));
        self.get_input();
    }

    /// Adds a line of dashes below the specified text
    fn convert_to_title(&self, text: &str) -> String {
        let chars = text.chars();
        let n = chars.count();
        let mut s = String::new();
        s.push('\n');
        s.push_str(text);
        s.push('\n');
        let mut s = helpers::add_repeated_char(s, '-', n);
        s.push('\n');
        s
    }
}

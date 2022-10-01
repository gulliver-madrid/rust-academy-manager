use {
    super::inner_console::InnerConsole,
    crate::{helpers, menus::shared, menus::shared::MenuItem},
    rust_i18n::t,
    std::rc::Rc,
};

pub struct UserInterface {
    pub inner_console: Rc<dyn InnerConsole>,
}

impl UserInterface {
    /// Requests input from the user and returns a MenuOption
    /// T is a enum that represents a menu option
    pub fn get_user_choice<'a, MenuOption>(
        &self,
        menu_items: &'a Vec<MenuItem<MenuOption>>,
    ) -> Option<&'a MenuOption> {
        let choice = self.get_input();
        shared::extract_option(choice, &menu_items)
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

    pub fn show(&self, text: String) {
        self.inner_console.show(&text);
    }

    pub fn show_title(&self, text: String) {
        self.show(self.convert_to_title(&text));
    }

    pub fn pause_enter(&self, text: &str) {
        self.show(format!("{} {}", t!("press_enter_to"), text));
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

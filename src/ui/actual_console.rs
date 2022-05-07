use rust_i18n::t;

use super::inner_console::InnerConsole;
use std::io;

const USE_SCREEN_CLEARING: bool = true;

pub struct ActualConsole;

impl InnerConsole for ActualConsole {
    fn clear_screen(&self) {
        if USE_SCREEN_CLEARING {
            clearscreen::clear().unwrap();
        }
    }
    fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect(&t!("errors.couldnt_read_user_input"));
        String::from(input.trim())
    }
    fn show(&self, text: &str) {
        println!("{}", text);
    }
}

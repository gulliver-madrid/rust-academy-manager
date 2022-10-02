#![cfg(test)]

use colored::*;
use std::fmt::Debug;

use crate::menus::shared::OptionText;

/// Returns the option number (indexing from 1)
pub fn choice_to_string<'a, MenuOption: PartialEq + Debug>(
    option: MenuOption,
    items_menu_data: &[(MenuOption, OptionText)],
) -> Option<String> {
    let mut index = 1;
    for item in items_menu_data {
        if item.0 == option {
            return Some(index.to_string());
        }
        index += 1;
    }
    None
}

/// Utility for improving test fail messages
pub fn highlight(s: &str) -> String {
    println!("En highlight hemos recibido {}", s);
    format!("\n\n{}: {}\n\n", "FAIL".red(), s.truecolor(255, 180, 0))
}

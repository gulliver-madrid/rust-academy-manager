#![cfg(test)]

use colored::*;
use std::fmt::Debug;

use crate::menus::shared::OptionText;

/// Returns the option number (indexing from 1)
pub fn choice_to_string<'a, MenuOption: PartialEq + Debug>(
    menu_option: MenuOption,
    menu_items: &[(MenuOption, OptionText)],
) -> Option<String> {
    let index = menu_items.iter().position(|item| item.0 == menu_option);
    index.map(|i| (i + 1).to_string())
}

/// Utility for improving test fail messages
pub fn highlight(msg: &str) -> String {
    let fail = "FAIL".red();
    let msg = msg.truecolor(255, 180, 0);
    format!("\n\n{fail}: {msg}\n\n")
}

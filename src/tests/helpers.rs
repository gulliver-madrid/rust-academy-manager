#![cfg(test)]

use colored::*;
use std::fmt::Debug;

use crate::menus::shared::OptionText;

/// Returns the option number (indexing from 1)
pub fn choice_to_string<'a, MenuOption: PartialEq + Debug>(
    menu_option: MenuOption,
    menu_items: &[(MenuOption, OptionText)],
) -> String {
    let index = menu_items
        .iter()
        .position(|item| item.0 == menu_option)
        .unwrap_or_else(|| {
            panic!("{}", err_msg_menu_option_not_found(menu_option, menu_items));
        });
    (index + 1).to_string()
}

/// Utility for improving test fail messages
pub fn highlight(msg: &str) -> String {
    let fail = "FAIL".red();
    let msg = msg.truecolor(255, 180, 0);
    format!("\n\n{fail}: {msg}\n\n")
}

fn err_msg_menu_option_not_found<MenuOption: Debug, MenuItems: Debug>(
    menu_option: MenuOption,
    menu_items: MenuItems,
) -> String {
    let fmt = format!("Menu option {menu_option:?} should exist in menu items, but it is {menu_items:?}");
    highlight(fmt.as_str())
}

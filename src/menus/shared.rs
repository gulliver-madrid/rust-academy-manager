use rust_i18n::t;

pub struct MenuItem<'a, MenuOption>
where
    MenuOption: Sized,
{
    pub text: &'a str,
    pub menu_option: MenuOption,
}

pub type OptionText = &'static str;

pub struct MenuExit;

/// Receive a string that should contain a number and from that number we will return the corresponding MenuOption, according to the order of the vector of MenuItem objects
pub fn extract_option<'a, MenuOption>(
    choice: String,
    menu_items: &'a Vec<MenuItem<MenuOption>>,
) -> Option<&'a MenuOption> {
    let option_number: usize = choice.parse().ok()?;
    let &chosen_item = &menu_items.get(option_number - 1)?;
    return Some(&chosen_item.menu_option);
}

/// Creates the text corresponding to the list of received MenuItem
pub fn create_options_text<T>(menu_items: &Vec<MenuItem<T>>) -> String {
    String::from(t!("choose_an_option") + ":\n")
        + &menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| format!("{} - {}", i + 1, t!(item.text)))
            .collect::<Vec<String>>()
            .join("\n")
}

/// Generates a vector of MenuItem from an array of tuples
/// items_menu_data is an array of tuples (MenuOption, OptionText)
/// MenuOption is a simple enum
/// OptionText is a static str
pub fn create_menu_items<'a, MenuOption, const N: usize>(
    items_menu_data: [(MenuOption, OptionText); N],
) -> Vec<MenuItem<'a, MenuOption>>
where
    MenuOption: Clone,
{
    items_menu_data
        .iter()
        .map(|(menu_option, text)| MenuItem {
            text,
            menu_option: menu_option.clone(),
        })
        .collect()
}

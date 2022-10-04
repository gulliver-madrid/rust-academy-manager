use rust_i18n::t;

pub struct MenuItem<MenuOption>
where
    MenuOption: Sized,
{
    pub text: &'static str,
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
    let chosen_item = menu_items.get(option_number - 1)?;
    return Some(&chosen_item.menu_option);
}

/// Creates the text corresponding to the list of given MenuItem
pub fn create_options_text<T>(menu_items: &Vec<MenuItem<T>>) -> String {
    let formatted_item_list = &menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{} - {}", i + 1, t!(item.text)))
        .collect::<Vec<String>>()
        .join("\n");
    format!("{}:\n{}", t!("choose_an_option"), formatted_item_list)
}

/// Generates a vector of MenuItem from an array of tuples
/// items_menu_data is an array of tuples (MenuOption, OptionText)
/// MenuOption is a simple enum
/// OptionText is a static str
pub fn create_menu_items<GenericMenuOption>(
    menu_items_data: &[(GenericMenuOption, OptionText)],
) -> Vec<MenuItem<GenericMenuOption>>
where
    GenericMenuOption: Clone,
{
    menu_items_data
        .iter()
        .map(|(menu_option, text)| MenuItem {
            text,
            menu_option: menu_option.clone(),
        })
        .collect()
}

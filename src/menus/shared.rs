use crate::texts;

pub struct MenuItem<'a, MenuOption> {
    pub text: &'a str,
    pub menu_option: MenuOption,
}

pub type OptionText = &'static str;

pub struct MenuExit;


pub fn extract_option<'a, MenuOption>(
    choice: String,
    menu_items: &'a Vec<MenuItem<MenuOption>>,
) -> Option<&'a MenuOption> {
    // Recibe una string que idealmente contendra un numero
    // y a partir de ese numero devolveremos la MenuOption
    // correspondiente, segun el orden del vector de objetos MenuItem
    let option_number: usize = choice.parse().ok()?;
    let &chosen_item = &menu_items.get(option_number - 1)?;
    return Some(&chosen_item.menu_option);
}

pub fn create_options_text<T>(menu_items: &Vec<MenuItem<T>>) -> String {
    // Crea el texto correspondiente a la lista de MenuItem recibidos
    String::from(texts::CHOOSE_AN_OPTION.to_owned() + ":\n")
        + &menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| format!("{} - {}", i + 1, item.text))
            .collect::<Vec<String>>()
            .join("\n")
}

/// Genera un vector de MenuItem a partir de un array de tuplas
pub fn create_menu_items<'a, MenuOption, const N: usize>(
    items_menu_data: [(MenuOption, OptionText); N],
) -> Vec<MenuItem<'a, MenuOption>>
where
    MenuOption: Clone,
{
    // items_menu_data es un array de tuplas (MenuOption, OptionText)
    // MenuOption es una enum simple
    // OptionText es una str estatica
    items_menu_data
        .iter()
        .map(|(menu_option, text)| MenuItem {
            text,
            menu_option: menu_option.clone(),
        })
        .collect()
}

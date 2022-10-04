#![macro_use]

macro_rules! count {
        () => (0usize);
        ( $one_token:tt $($token:tt)* ) => (1usize + count!($($token)*));
    }

/// It defines MenuOption, MenuItems and MENU_ITEMS_DATA
/// MenuOption: it's an enum
/// MenuItems: it's a type
/// MENU_ITEMS_DATA: it's an const array
/// It depends on MenuItem
macro_rules! create_menu_options {

    ($(($first:ident, $second:expr)),+) => {

        use crate::menus::shared::MenuItem;

        const N: usize = count!($($first)+);

        #[derive(Debug, Clone, PartialEq)]
        pub enum MenuOption {
            $($first, )+
        }

        type MenuItemsDataType = [(MenuOption, &'static str); N];

        pub static MENU_ITEMS_DATA: MenuItemsDataType = [
            $( (MenuOption::$first, $second), )+
        ];
        type MenuItems = Vec<MenuItem<MenuOption>>;

    };
}

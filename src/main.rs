mod application;
mod components;
mod config;
mod domain;
mod errors;
mod helpers;
mod menus;
mod repository;
mod tests;
mod ui;
mod views;

use std::rc::Rc;

use application::create_application;
use rust_i18n::t;
use ui::UserInterface;

use crate::{
    components::Control, menus::MainMenu, repository::JsonPersistence, ui::ActualConsole,
};

rust_i18n::i18n!("locales");

fn main() {
    let matches = config::get_arg_matches();
    let language = config::get_language(&matches);

    rust_i18n::set_locale(&language);

    let data_folder = String::from(matches.value_of("data").unwrap_or(""));

    let persistence = JsonPersistence {
        project_dir: data_folder,
    };
    let ui = ui::UserInterface {
        inner_console: Rc::new(ActualConsole),
    };
    let ok = option_to_create_data_path(&persistence, &ui);
    if ok {
        let control = build_control(persistence, ui);
        start_app(Rc::new(control));
    }
    println!("\n{}\n", t!("program_finished"));
}

fn start_app<'a>(control: Rc<Control>) {
    let mut menu = MainMenu::new(control);
    menu.open_menu();
}

fn option_to_create_data_path(persistence: &JsonPersistence, ui: &UserInterface) -> bool {
    if !persistence.data_path_exists() {
        let path = persistence.get_project_data_path();
        let path = match path.to_str() {
            Some(path) => format!("'{}'", path),
            None => {
                ui.show(t!("errors.trying_convert_data_path"));
                return false;
            }
        };
        ui.show(t!("option_to_create_data_path", path = &path).to_string());
        let option_yes = t!("option_yes_one_char");
        if ui.get_input().as_str() == option_yes {
            persistence.create_data_folder();
            return true;
        }
        return false;
    }
    true
}
fn build_control(persistence: JsonPersistence, ui: UserInterface) -> Control {
    let application = create_application(Rc::new(persistence));
    Control { ui, application }
}

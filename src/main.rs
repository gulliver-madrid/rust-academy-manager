mod application;
mod components;
mod domain;
mod errors;
mod helpers;
mod menus;
mod repository;
mod tests;
mod ui;
mod views;

use clap::{arg, command, ArgMatches};
use rust_i18n::t;

use crate::{
    application::Application, components::Control, menus::MainMenu,
    repository::JsonPersistence, ui::ActualConsole,
};

const ALLOWED_LANGUAGES: [&str; 2] = ["es", "en"];
const DEFAULT_LANGUAGE: &str = "en";

rust_i18n::i18n!("locales");

fn get_arg_matches() -> ArgMatches {
    let matches = command!()
        .arg(arg!(--data <PATH> "data folder").required(false).short('d'))
        .arg(
            arg!(--lang <LANGUAGE> "language")
                .required(false)
                .short('l'),
        )
        .get_matches();
    matches
}

fn main() {
    let matches = get_arg_matches();
    let mut language = String::from(matches.value_of("lang").unwrap_or(DEFAULT_LANGUAGE));
    if !is_valid_language(&language) {
        println!("{}", create_unknown_lang_error_text(&language));
        language = DEFAULT_LANGUAGE.to_string();
    };
    rust_i18n::set_locale(&language);
    let data_folder = String::from(matches.value_of("data").unwrap_or(""));

    let persistence = JsonPersistence {
        project_dir: data_folder,
    };
    let ui = ui::UserInterface {
        inner_console: Box::new(ActualConsole {}),
    };
    if !persistence.data_path_exists() {
        ui.show(t!("option_to_create_data_path").to_string());
        let option_yes = t!("option_yes_one_char");
        if ui.get_input().as_str() == option_yes {
            persistence.create_data_folder();
            let application = Application::new(Box::new(persistence));
            let mut control = Control { ui, application };
            let mut menu = MainMenu::new(&mut control);
            menu.open_menu();
        }
    }
    println!("\n{}\n", t!("program_finished"));
}

fn is_valid_language(language: &str) -> bool {
    ALLOWED_LANGUAGES.contains(&language)
}

fn create_unknown_lang_error_text(language: &str) -> String {
    format!(
        "Error: Unknown language: {}. Available languages: {}. Default to english",
        language,
        ALLOWED_LANGUAGES.join(", ")
    )
}

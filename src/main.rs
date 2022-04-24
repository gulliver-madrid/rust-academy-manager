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

use menus::MainMenu;
use rust_i18n::t;

use crate::{
    application::Application, components::Control, repository::JsonPersistence,
    ui::ActualConsole,
};

rust_i18n::i18n!("locales");

use clap::{arg, command};

const ALLOWED_LANGUAGES: [&str; 2] = ["es", "en"];

fn main() {
    let matches = command!()
        .arg(arg!(--data <PATH> "data folder").required(false).short('d'))
        .arg(arg!(--lang <PATH> "language").required(false).short('l'))
        .get_matches();
    let language = String::from(matches.value_of("lang").unwrap_or("en"));
    if !(ALLOWED_LANGUAGES.contains(&language.as_ref())) {
        println!("Error: Unknown language: {}. Available languages: {}", language, ALLOWED_LANGUAGES.join(", "));
        return;
    };
    rust_i18n::set_locale(&language);
    let data_folder = String::from(matches.value_of("data").unwrap_or(""));

    let persistence = JsonPersistence {
        project_dir: data_folder,
    };
    let application = Application::new(Box::new(persistence));
    let ui = ui::UserInterface {
        inner_console: Box::new(ActualConsole {}),
    };
    let mut control = Control {
        ui: ui,
        application,
    };
    let mut menu = MainMenu::new(&mut control);
    menu.open_menu();
    println!("\n{}\n", t!("program_finished"));
}

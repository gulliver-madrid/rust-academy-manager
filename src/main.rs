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

fn main() {
    rust_i18n::set_locale("en");
    let persistence = JsonPersistence {};
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

mod application;
mod components;
mod domain;
mod errors;
mod helpers;
mod menus;
mod repository;
mod tests;
mod texts;
mod ui;
mod views;

use menus::MainMenu;

use crate::{
    application::Application, components::Control, repository::Persistence,
    ui::ActualConsole,
};

fn main() {
    let persistencia = Persistence {};
    let application = Application::new(Box::new(persistencia));
    let ui = ui::UserInterface {
        inner_console: Box::new(ActualConsole {}),
    };
    let mut control = Control {
        ui: ui,
        application,
    };
    let mut menu = MainMenu::new(&mut control);
    menu.open_menu();
    println!("\nPrograma finalizado\n");
}

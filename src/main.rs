mod application;
mod components;
mod dominio;
mod errors;
mod helpers;
mod menus;
mod repository;
mod tests;
mod textos;
mod ui;
mod views;

use menus::MenuPrincipal;

use crate::{
    application::Application, components::Control, repository::Persistencia,
    ui::ActualConsole,
};

fn main() {
    let persistencia = Persistencia {};
    let application = Application::new(Box::new(persistencia));
    let ui = ui::UserInterface {
        inner_console: Box::new(ActualConsole {}),
    };
    let mut control = Control {
        ui: ui,
        application,
    };
    let mut menu = MenuPrincipal::new(&mut control);
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

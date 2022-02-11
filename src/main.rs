mod application;
mod components;
mod consola;
mod dominio;
mod errors;
mod helpers;
mod menus;
mod repository;
mod tests;
mod textos;
mod views;

use menus::{ MenuPrincipal};

use crate::{
    application::Application, components::Control, consola::ActualConsole,
    repository::Persistencia,
};

fn main() {
    let persistencia = Persistencia {};
    let application = Application::new(Box::new(persistencia));
    let consola = consola::Consola {
        inner_console: Box::new(ActualConsole {}),
    };
    let mut control = Control {
        consola,
        application,
    };
    let mut menu = MenuPrincipal::new(&mut control);
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

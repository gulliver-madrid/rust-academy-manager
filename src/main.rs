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

use menus::{Menu, MenuPrincipal};

use crate::{application::Application, components::Control};

fn main() {
    let application = Application::new();
    let consola = consola::Consola {};
    let mut control = Control {
        consola,
        application,
    };
    let mut menu = MenuPrincipal {};
    menu.abrir_menu(&mut control);
    println!("\nPrograma finalizado\n");
}

mod consola;
mod dominio;
mod helpers;
mod repo;
mod tests;

mod components;
mod menus;

mod serializable;
mod serialization;
mod textos;
mod views;

use menus::{Menu, MenuPrincipal};

use crate::{components::Control, repo::Repository};

fn main() {
    let repository = Repository {};
    let consola = consola::Consola {};
    let control = Control {
        consola,
        repository,
    };
    let mut menu = MenuPrincipal {};
    menu.abrir_menu(&control);
    println!("\nPrograma finalizado\n");
}

mod consola;
mod dominio;
mod helpers;
mod persistencia;
mod tests;

mod components;
mod menus;

mod serializable;
mod serialization;
mod textos;
mod views;

use menus::{Menu, MenuPrincipal};

use crate::{components::Control, persistencia::Persistencia};

fn main() {
    let persistencia = Persistencia {};
    let consola = consola::Consola {};
    let control = Control {
        consola,
        persistencia,
    };
    let mut menu = MenuPrincipal {};
    menu.abrir_menu(&control);
    println!("\nPrograma finalizado\n");
}

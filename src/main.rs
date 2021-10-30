mod consola;
mod dominio;
mod helpers;
mod menus;
mod repo;
mod serializable;
mod serialization;
mod textos;
mod views;

use menus::{Menu, MenuPrincipal};

fn main() {
    let consola = consola::Consola {};
    let mut menu = MenuPrincipal { consola: &consola };
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

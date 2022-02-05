mod components;
mod consola;
mod dominio;
mod helpers;
mod menus;
mod persistencia;
mod repository;
mod serializable;
mod serialization;
mod tests;
mod textos;
mod views;

use menus::{Menu, MenuPrincipal};

use crate::{
    components::Control,
    persistencia::Persistencia,
    repository::{Modelo, Repository},
};

fn main() {
    let persistencia = Persistencia {};
    let repository = Repository {
        persistencia,
        modelo: Modelo {},
    };
    let consola = consola::Consola {};
    let control = Control {
        consola,
        repository,
    };
    let mut menu = MenuPrincipal {};
    menu.abrir_menu(&control);
    println!("\nPrograma finalizado\n");
}

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

use crate::{
    components::Control,
    repository::persistencia::Persistencia,
    repository::{modelo::Modelo, Application, Repository},
};

fn main() {
    let persistencia = Persistencia {};
    let repository = Repository {
        persistencia,
        modelo: Modelo { profesores: None },
    };
    let application = Application { repository };
    let consola = consola::Consola {};
    let mut control = Control {
        consola,
        application,
    };
    let mut menu = MenuPrincipal {};
    menu.abrir_menu(&mut control);
    println!("\nPrograma finalizado\n");
}

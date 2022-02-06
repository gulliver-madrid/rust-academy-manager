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

use crate::{
    application::Application,
    components::Control,
    repository::{modelo::Modelo, Persistencia, Repository},
};

fn main() {
    let persistencia = Persistencia {};
    let repository = Repository {
        persistencia,
        modelo: Modelo { profesores: None, asignaturas:None },
    };
    let application = Application::new(repository);
    let consola = consola::Consola {};
    let mut control = Control {
        consola,
        application,
    };
    let mut menu = MenuPrincipal {};
    menu.abrir_menu(&mut control);
    println!("\nPrograma finalizado\n");
}

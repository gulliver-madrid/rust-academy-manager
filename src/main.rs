mod asignatura;
mod consola;
mod helpers;
mod teachers;
mod menus;
mod repo;
mod serializable;
mod serialization;
mod textos;
mod views;

use menus::{Menu, MenuPrincipal};

fn main() {
    println!("\nPROFESORES\n");
    let consola = consola::Consola {};
    let mut menu = MenuPrincipal { consola: &consola };
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

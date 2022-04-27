mod asignatura;
mod helpers;
mod menus;
mod repo;
mod serializable;
mod serialization;
mod teachers;
mod textos;
mod views;
mod vista;

use menus::{Menu, MenuPrincipal};

fn main() {
    println!("\nPROFESORES\n");
    let vista = vista::Vista {};
    let menu = MenuPrincipal { vista: &vista };
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

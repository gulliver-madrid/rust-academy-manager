mod asignatura;
mod helpers;
mod menu_asignaturas;
mod menu_principal;
mod menu_profes;
mod repo;
mod serializable;
mod serialization;
mod shared_menus;
mod teachers;
mod textos;
mod views;
mod vista;

use menu_principal::MenuPrincipal;
use shared_menus::Menu;

fn main() {
    println!("\nPROFESORES\n");
    let vista = vista::Vista {};
    let menu = MenuPrincipal { vista: &vista };
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

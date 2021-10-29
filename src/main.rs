
mod asignatura;
mod helpers;
mod menu_asignaturas;
mod menu_principal;
mod menu_profes;
mod repo;
mod serializable;
mod serialization;
mod teachers;
mod shared_menus;
mod textos;
mod views;
mod vista;

fn main() {
    println!("\nPROFESORES\n");
    let menu = menu_principal::MenuPrincipal {};
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

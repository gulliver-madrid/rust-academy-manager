mod helpers;
mod menu_principal;
mod menu_profes;
mod repo;
mod serializable;
mod serialization;
mod teachers;
mod textos;
mod views;
mod vista;

fn main() {
    println!("\nPROFESORES\n");
    let menu = menu_principal::MenuPrincipal {};
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

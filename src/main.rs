mod helpers;
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
    let menu = menu_profes::MenuProfesores {};
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

mod helpers;
mod repo;
mod serializable;
mod serialization;
mod teachers;
mod textos;
mod vista;

use helpers::set_number_chars;
use teachers::{Profesor, Profesores};

fn main() {
    println!("\nPROFESORES\n");
    let menu = MenuProfesores {};
    menu.abrir_menu();
    println!("\nPrograma finalizado\n");
}

struct MenuProfesores;

impl MenuProfesores {
    fn abrir_menu(&self) {
        let mut profesores = repo::get_profesores();
        let vista = vista::Vista {};
        loop {
            vista.mostrar(textos::OPCIONES_MENU_PRINCIPAL);
            let nombre = vista.get_input();
            let eleccion = nombre.trim();
            match eleccion {
                "1" => self.mostrar_lista_profes(&profesores, &vista),
                "2" => self.abrir_menu_anadir_profe(&mut profesores, &vista),
                "3" => return,
                _ => continue,
            }
        }
    }

    fn mostrar_lista_profes(
        &self,
        profesores: &Profesores,
        vista: &vista::Vista,
    ) {
        for profe in profesores {
            vista.mostrar(&crear_linea_tabla_profesor(profe));
        }
    }

    fn abrir_menu_anadir_profe(
        &self,
        profesores: &mut Profesores,
        vista: &vista::Vista,
    ) {
        let new_id: u32;
        {
            let last_index = profesores.len() - 1;
            let last_profe = profesores.get(last_index).unwrap().clone();
            new_id = last_profe.id + 1;
        }

        vista.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
        let nombre = vista.get_input();
        if nombre.trim() == "" {
            return;
        } else {
            let nuevo_profe = Profesor {
                nombre,
                id: new_id,
                telefono: String::new(),
            };
            profesores.push(nuevo_profe);

            repo::save_profesores(profesores.clone());
        }
    }
}

fn crear_linea_tabla_profesor(profe: &Profesor) -> String {
    let profe_str = set_number_chars(&profe.nombre, 22);
    let tlf_str = match profe.telefono.as_str() {
        "" => "desconocido",
        otro => otro,
    };
    format!(
        "Nombre: {}  Id: {}  Telefono: {}",
        profe_str,
        format!("{:0>3}", profe.id),
        tlf_str
    )
}

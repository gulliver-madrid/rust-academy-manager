use crate::teachers::{Profesor, Profesores};
use crate::repo;
use crate::textos;
use crate::views::View;
use crate::vista;


pub struct MenuProfesores;

impl MenuProfesores {
    pub fn abrir_menu(&self) {
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
            vista.mostrar(&profe.crear_linea_tabla());
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

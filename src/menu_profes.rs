use crate::teachers::{Profesor, Profesores};
use crate::helpers;
use crate::repo;
use crate::textos;
use crate::views::View;
use crate::vista;

pub struct MenuProfesores;

impl MenuProfesores {
    pub fn abrir_menu(&self, vista: &vista::Vista) {
        let mut profesores = repo::get_profesores();
        loop {
            vista.mostrar(textos::OPCIONES_MENU_PROFESORES);
            let eleccion = vista.get_input();
            match eleccion.as_str() {
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
            let last_profe = helpers::get_last_element(profesores);
            match last_profe {
                None => {
                    vista.mostrar("Error: no se encontró ningún profesor");
                    return;
                }
                Some(last_profe) => new_id = last_profe.id + 1,
            }
        }

        vista.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
        let nombre_introducido = vista.get_input();
        match nombre_introducido.as_str() {
            "" => return,
            _ => {
                let nuevo_profe = Profesor {
                    nombre: String::from(nombre_introducido),
                    id: new_id,
                    telefono: String::new(),
                };
                profesores.push(nuevo_profe);
                repo::save_profesores(profesores.clone());
            }
        }
    }
}

use crate::{components::Control};

use super::Menu;

pub struct MenuAsignarProfesor {}

impl MenuAsignarProfesor {
    fn _abrir_menu(&mut self, control: &mut Control) {
        let consola = &control.consola;
        consola.mostrar("Elige la asignatura a la que quieras asignar profesor");
        let nombre_asignatura: String;
        match consola.pide_texto_a_usuario() {
            Some(entrada) => nombre_asignatura = entrada,
            None => {
                consola.mostrar("Operación cancelada");
                consola.pausa_enter("continuar");
                return;
            }
        }
        let index_asignatura: usize;
        match control
            .application
            .get_subject_index_by_name(&nombre_asignatura)
        {
            Ok(index) => {
                index_asignatura = index;
            }
            Err(e) => {
                consola.mostrar(&e.to_string());
                consola.pausa_enter("continuar");
                return;
            }
        }
        consola.mostrar(&format!("Asignatura introducida: {}", nombre_asignatura));
        consola.mostrar("Introduce el ID del profesor");
        if let Some(entrada) = consola.pide_texto_a_usuario() {
            let id_profesor = entrada.parse::<u32>().unwrap();
            let result = control
                .application
                .asignar_profesor_a_asignatura(index_asignatura, id_profesor);
            if result.is_ok() {
                consola.mostrar("Profesor asignado correctamente");
            } else {
                consola.mostrar("No se pudo realizar la operación");
            }
            consola.pausa_enter("continuar");
        }
    }
}

impl Menu for MenuAsignarProfesor {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

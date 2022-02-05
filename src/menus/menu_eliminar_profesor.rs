use crate::{components::Control, textos};

use super::Menu;

pub struct MenuEliminarProfesor {}

impl Menu for MenuEliminarProfesor {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

impl MenuEliminarProfesor {
    fn _abrir_menu(&mut self, control: &mut Control) {
        self.mostrar_texto_menu(control);
        match control.consola.pide_texto_a_usuario() {
            None => (),
            Some(nombre) => {
                self.eliminar_profesor(nombre, control);
                control.consola.pausa_enter("continuar");
            }
        }
    }

    fn mostrar_texto_menu(&self, control: &mut Control) {
        control
            .consola
            .mostrar(textos::INTRODUCE_NOMBRE_PROFESOR_A_ELIMINAR);
    }

    fn eliminar_profesor(&mut self, nombre: String, control: &mut Control) {
        let profesores = &mut control.repository.modelo.profesores.as_mut().unwrap();
        match profesores.iter().position(|a| a.nombre == nombre) {
            Some(index) => {
                profesores.remove(index);
                control.repository.persistencia.save_profesores(profesores);
            }
            None => control
                .consola
                .mostrar(&format!("No hay ninguna profesor con el nombre {}", nombre)),
        }
    }
}

use crate::{components::Control, dominio::teachers::Profesores, textos};

use super::Menu;

pub struct MenuEliminarProfesor<'a> {
    profesores: &'a mut Profesores,
}

impl Menu for MenuEliminarProfesor<'_> {
    fn abrir_menu(&mut self, control: &Control) {
        self._abrir_menu(control);
    }
}

impl MenuEliminarProfesor<'_> {
    pub fn new<'a>(profesores: &'a mut Profesores) -> MenuEliminarProfesor<'a> {
        MenuEliminarProfesor { profesores }
    }

    fn _abrir_menu(&mut self, control: &Control) {
        self.mostrar_texto_menu(control);
        match control.consola.pide_texto_a_usuario() {
            None => (),
            Some(nombre) => {
                self.eliminar_profesor(nombre, control);
                control.consola.pausa_enter("continuar");
            }
        }
    }

    fn mostrar_texto_menu(&self, control: &Control) {
        control
            .consola
            .mostrar(textos::INTRODUCE_NOMBRE_PROFESOR_A_ELIMINAR);
    }

    fn eliminar_profesor(&mut self, nombre: String, control: &Control) {
        match self.profesores.iter().position(|a| a.nombre == nombre) {
            Some(index) => {
                self.profesores.remove(index);
                control.repository.save_profesores(&self.profesores);
            }
            None => control
                .consola
                .mostrar(&format!("No hay ninguna profesor con el nombre {}", nombre)),
        }
    }
}

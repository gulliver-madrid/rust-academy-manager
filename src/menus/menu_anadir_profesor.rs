use crate::{components::Control, consola::Consola, textos};

use super::Menu;

pub struct MenuAnadirProfesor {}

impl MenuAnadirProfesor {
    fn _abrir_menu(&mut self, control: &mut Control) {
        self.mostrar_texto_menu(&control.consola);
        match control.consola.pide_texto_a_usuario() {
            None => return,
            Some(nombre) => {
                control.repository.anadir_nuevo_profesor(nombre);
            }
        }
    }

    fn mostrar_texto_menu(&self, consola: &Consola) {
        consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
    }
}

impl Menu for MenuAnadirProfesor {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

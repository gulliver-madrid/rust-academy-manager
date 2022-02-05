use crate::{components::Control, dominio::Profesor, helpers, textos};

use super::Menu;

pub struct MenuAnadirProfesor {}
impl MenuAnadirProfesor {
    fn _abrir_menu(&mut self, control: &mut Control) {
        self.mostrar_texto_menu(control);
        match control.consola.pide_texto_a_usuario() {
            None => return,
            Some(nombre) => {
                let next_id: u32 = self._get_next_id(control);
                let nuevo_profesor = self._crear_nuevo_profesor(nombre, next_id);
                self._anadir_profesor(nuevo_profesor, control);
            }
        }
    }

    fn mostrar_texto_menu(&self, control: &mut Control) {
        control.consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
    }

    fn _anadir_profesor(&mut self, profesor: Profesor, control: &mut Control) {
        let profesores = &mut control.repository.modelo.profesores.as_mut().unwrap();
        profesores.push(profesor);
        control.repository.persistencia.save_profesores(profesores);
    }

    fn _get_next_id(&self, control: &Control) -> u32 {
        let profesores = &control.repository.modelo.profesores.as_ref().unwrap();
        let last_profesor =
            helpers::get_last_element(profesores).expect(textos::errores::NO_PROFESOR);
        last_profesor.id + 1
    }

    fn _crear_nuevo_profesor(&self, nombre: String, id: u32) -> Profesor {
        Profesor {
            nombre,
            id,
            telefono: String::new(),
        }
    }
}

impl Menu for MenuAnadirProfesor {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

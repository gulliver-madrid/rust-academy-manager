use crate::{
    consola,
    dominio::{teachers::Profesores, Profesor},
    helpers, repo, textos,
};

use super::Menu;

pub struct MenuAnadirProfesor<'a> {
    consola: &'a consola::Consola,
    profesores: &'a mut Profesores,
}
impl MenuAnadirProfesor<'_> {
    pub fn new<'a>(
        consola: &'a consola::Consola,
        profesores: &'a mut Profesores,
    ) -> MenuAnadirProfesor<'a> {
        MenuAnadirProfesor {
            consola: consola,
            profesores,
        }
    }

    fn _abrir_menu(&mut self) {
        let next_id: u32 = self.get_next_id();
        self.mostrar_texto_menu();
        match self.consola.pide_texto_a_usuario() {
            None => return,
            Some(nombre) => {
                let nuevo_profe = self.crear_nuevo_profe(nombre, next_id);
                self.anadir_profe(nuevo_profe);
            }
        }
    }
    fn mostrar_texto_menu(&self) {
        self.consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
    }

    fn anadir_profe(&mut self, profesor: Profesor) {
        self.profesores.push(profesor);
        repo::save_profesores(self.profesores.clone());
    }

    fn get_next_id(&self) -> u32 {
        let last_profe = helpers::get_last_element(&self.profesores) //
            .expect(textos::ERROR_NO_PROFESOR);
        last_profe.id + 1
    }

    fn crear_nuevo_profe(&self, nombre: String, id: u32) -> Profesor {
        Profesor {
            nombre,
            id,
            telefono: String::new(),
        }
    }
}

impl Menu for MenuAnadirProfesor<'_> {
    fn abrir_menu(&mut self) {
        self._abrir_menu();
    }
}

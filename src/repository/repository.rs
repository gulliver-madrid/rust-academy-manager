
use super::persistencia::Persistencia;

use super::{add_teacher::AddTeacherUseCase, modelo::Modelo};

pub struct Repository {
    pub persistencia: Persistencia,
    pub modelo: Modelo,
}

impl Repository {
    pub fn load_profesores(&mut self) {
        match self.modelo.profesores {
            None => {
                self.modelo.profesores = Some(self.persistencia.load_profesores());
            }
            _ => {}
        }
    }

    pub fn anadir_nuevo_profesor(&mut self, nombre: String) {
        AddTeacherUseCase {
            persistencia: &mut self.persistencia,
            modelo: &mut self.modelo,
        }.anadir_nuevo_profesor(nombre);
    }
}

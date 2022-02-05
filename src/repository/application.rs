use crate::errors::SimpleError;

use super::{
    add_teacher::AddTeacherUseCase, remove_teacher::RemoveTeacherUseCase,
    repository::Repository,
};

pub struct Application {
    pub repository: Repository,
}

impl Application {
    pub fn anadir_nuevo_profesor(&mut self, nombre: String) {
        AddTeacherUseCase {
            repository: &mut self.repository,
        }
        .anadir_nuevo_profesor(nombre);
    }
    pub fn eliminar_profesor(&mut self, nombre: &str) -> Result<(), SimpleError> {
        RemoveTeacherUseCase {
            repository: &mut self.repository,
        }
        .eliminar_profesor(nombre.to_string())
    }
}

use crate::{errors::SimpleResult, repository::Repository};

use super::{add_teacher::AddTeacherUseCase, remove_teacher::RemoveTeacherUseCase};

pub struct Application {
    pub repository: Repository,
}

impl Application {
    pub fn anadir_nuevo_profesor(&mut self, nombre: &str) -> SimpleResult {
        AddTeacherUseCase {
            repository: &mut self.repository,
        }
        .anadir_nuevo_profesor(nombre.to_string())
    }
    pub fn eliminar_profesor(&mut self, nombre: &str) -> SimpleResult {
        RemoveTeacherUseCase {
            repository: &mut self.repository,
        }
        .eliminar_profesor(nombre.to_string())
    }
}

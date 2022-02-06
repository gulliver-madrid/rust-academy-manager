use crate::{errors::SimpleResult, repository::Repository};

use super::{add_teacher::AddTeacherUseCase, remove_teacher::RemoveTeacherUseCase, add_subject::AddSubjectUseCase, remove_subject::RemoveSubjectUseCase};

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
    pub fn add_new_subject(&mut self, nombre: &str) -> SimpleResult {
        AddSubjectUseCase {
            repository: &mut self.repository,
        }
        .add_new_subject(nombre.to_string())
    }
    pub fn remove_subject(&mut self, nombre: &str) -> SimpleResult {
        RemoveSubjectUseCase {
            repository: &mut self.repository,
        }
        .remove_subject(nombre.to_string())
    }
}

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
};

use super::{
    add_subject::AddSubjectUseCase, add_teacher::AddTeacherUseCase,
    remove_subject::RemoveSubjectUseCase, remove_teacher::RemoveTeacherUseCase, assign_teacher_to_subject::AssignTeacherToSubjectUseCase, get_subject_index_by_name::GetSubjectIndexByNameUseCase,
};

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

    pub fn get_subject_index_by_name(
        &mut self,
        nombre_asignatura: &str,
    ) -> Result<usize, SimpleError> {
        GetSubjectIndexByNameUseCase {
            repository: &mut self.repository,
        }
        .get_subject_index_by_name(nombre_asignatura)
    }

    pub fn asignar_profesor_a_asignatura(
        &mut self,
        index_asignatura: usize,
        id_profesor: u32,
    ) -> SimpleResult {
        AssignTeacherToSubjectUseCase {
            repository: &mut self.repository,
        }
        .asignar_profesor_a_asignatura(index_asignatura, id_profesor)
    }
}

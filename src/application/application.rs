use crate::{
    dominio::{Asignaturas, Profesores},
    errors::{SimpleError, SimpleResult},
    repository::Repository,
};

use super::{
    add_subject::AddSubjectUseCase, add_teacher::AddTeacherUseCase,
    assign_teacher_to_subject::AssignTeacherToSubjectUseCase,
    get_subject_index_by_name::GetSubjectIndexByNameUseCase,
    remove_subject::RemoveSubjectUseCase, remove_teacher::RemoveTeacherUseCase,
};

pub struct Application {
    repository: Repository,
}

impl Application {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }
    /// Carga las asignaturas en el Modelo si es necesario
    pub fn load_subjects(&mut self) -> () {
        self.repository.load_subjects();
    }
    /// Carga los profesores en el Modelo si es necesario
    pub fn load_profesores(&mut self) -> () {
        self.repository.load_profesores();
    }
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
    pub fn get_asignaturas(&self) -> &Asignaturas {
        &self.repository.modelo.asignaturas.as_ref().unwrap()
    }
    pub fn get_teachers(&self) -> &Profesores {
        &self.repository.modelo.profesores.as_ref().unwrap()
    }
}

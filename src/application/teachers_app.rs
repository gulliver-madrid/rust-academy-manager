use std::{cell::RefCell, rc::Rc};

use crate::{domain::Teachers, errors::SimpleResult, repository::Repository};

use super::usecases::{AddTeacherUseCase, RemoveTeacherUseCase};

/// Permite interactuar con los repositorios mediante operaciones predefinidas relacionadas con los profesores.
pub struct TeachersApp {
    repository: Rc<RefCell<Repository>>,
}


impl TeachersApp {
    pub fn new(repo_ref: &Rc<RefCell<Repository>>) -> Self {
        Self {
            repository: Rc::clone(repo_ref),
        }
    }

    /// Carga los profesores en el Modelo si es necesario.
    pub fn load_teachers_if_needed(&mut self) -> () {
        self.repository.borrow_mut().load_teachers_if_needed();
    }

    /// Devuelve una copia de la lista de profesores.
    pub fn get_teachers(&self) -> Teachers {
        self.repository
            .borrow()
            .model
            .teachers
            .as_ref()
            .unwrap()
            .clone()
    }

    /// Añade un nuevo profesor con el nombre especificado.
    pub fn add_new_teacher(&mut self, name: &str) -> SimpleResult {
        AddTeacherUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .add_new_teacher(name.to_string())
    }

    /// Elimina un profesor identificado por su nombre.
    pub fn remove_teacher(&mut self, name: &str) -> SimpleResult {
        RemoveTeacherUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .remove_teacher(name.to_string())
    }
}

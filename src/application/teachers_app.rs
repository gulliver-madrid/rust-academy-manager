use std::{cell::RefCell, rc::Rc};

use crate::{dominio::Profesores, errors::SimpleResult, repository::Repository};

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
    pub fn load_profesores(&mut self) -> () {
        self.repository.borrow_mut().load_profesores();
    }

    /// Devuelve una copia de la lista de profesores.
    pub fn get_teachers(&self) -> Profesores {
        self.repository
            .borrow()
            .modelo
            .profesores
            .as_ref()
            .unwrap()
            .clone()
    }

    /// Añade un nuevo profesor con el nombre especificado.
    pub fn anadir_nuevo_profesor(&mut self, nombre: &str) -> SimpleResult {
        AddTeacherUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .anadir_nuevo_profesor(nombre.to_string())
    }

    /// Elimina un profesor identificado por su nombre.
    pub fn eliminar_profesor(&mut self, nombre: &str) -> SimpleResult {
        RemoveTeacherUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .eliminar_profesor(nombre.to_string())
    }
}

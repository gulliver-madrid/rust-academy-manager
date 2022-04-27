use std::{cell::RefCell, rc::Rc};

use crate::{
    dominio::Asignaturas,
    errors::{SimpleError, SimpleResult},
    repository::{create_repo, Repository},
};

use super::{
    teachers_app::TeachersApp,
    usecases::{
        AddSubjectUseCase, AssignTeacherToSubjectUseCase,
        GetSubjectIndexByNameUseCase, RemoveSubjectUseCase,
    },
};

pub struct Application {
    repository: Rc<RefCell<Repository>>,
    pub teachers_app: TeachersApp,
}

impl Application {
    pub fn new() -> Self {
        let repository = create_repo();
        let repo_ref = Rc::new(RefCell::new(repository));
        let teachers_app = TeachersApp::new(&repo_ref);
        Self {
            repository: Rc::clone(&repo_ref),
            teachers_app,
        }
    }

    /// Carga las asignaturas en el Modelo si es necesario.
    pub fn load_subjects(&mut self) -> () {
        self.repository.borrow_mut().load_subjects();
    }

    /// Devuelve una copia de la lista de asignaturas.
    pub fn get_asignaturas(&self) -> Asignaturas {
        self.repository
            .borrow()
            .modelo
            .asignaturas
            .as_ref()
            .unwrap()
            .clone()
    }

    /// Añade una nuevo asignatura con el nombre especificado.
    pub fn add_new_subject(&mut self, nombre: &str) -> SimpleResult {
        AddSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .add_new_subject(nombre.to_string())
    }

    /// Elimina una asignatura identificada por su nombre.
    pub fn remove_subject(&mut self, nombre: &str) -> SimpleResult {
        RemoveSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .remove_subject(nombre.to_string())
    }

    /// Devuelve el índice de la asignatura con el nombre especificado.
    pub fn get_subject_index_by_name(
        &mut self,
        nombre_asignatura: &str,
    ) -> Result<usize, SimpleError> {
        GetSubjectIndexByNameUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .get_subject_index_by_name(nombre_asignatura)
    }

    /// Asigna un profesor identificado por su id a la asignatura con el indice especificado.
    pub fn asignar_profesor_a_asignatura(
        &mut self,
        index_asignatura: usize,
        id_profesor: u32,
    ) -> SimpleResult {
        AssignTeacherToSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .asignar_profesor_a_asignatura(index_asignatura, id_profesor)
    }
}

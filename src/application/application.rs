use std::{cell::RefCell, rc::Rc};

use rust_i18n::t;

use crate::{
    domain::Subjects,
    errors::{SimpleError, SimpleResult},
    repository::{create_repo, PersistenceTrait, Repository},
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
    pub fn new(persistencia: Box<dyn PersistenceTrait>) -> Self {
        let repository = create_repo(persistencia);
        let repo_ref = Rc::new(RefCell::new(repository));
        let teachers_app = TeachersApp::new(&repo_ref);
        Self {
            repository: Rc::clone(&repo_ref),
            teachers_app,
        }
    }

    /// Carga las asignaturas en el Modelo si es necesario.
    pub fn load_subjects_if_needed(&mut self) -> () {
        self.repository.borrow_mut().load_subjects_if_needed();
    }

    /// Devuelve una copia de la lista de asignaturas.
    pub fn get_subjects(&self) -> Result<Subjects, SimpleError> {
        let option = self.repository.borrow().model.subjects.clone();
        option.ok_or(SimpleError::new(&t!("couldnt_get_subjects")))
    }

    /// Añade una nuevo asignatura con el nombre especificado.
    pub fn add_new_subject(&mut self, name: &str) -> SimpleResult {
        AddSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .add_new_subject(name.to_string())
    }

    /// Elimina una asignatura identificada por su nombre.
    pub fn remove_subject(&mut self, name: &str) -> SimpleResult {
        RemoveSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .remove_subject(name.to_string())
    }

    /// Devuelve el índice de la asignatura con el nombre especificado.
    pub fn get_subject_index_by_name(
        &mut self,
        subject_name: &str,
    ) -> Result<usize, SimpleError> {
        GetSubjectIndexByNameUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .get_subject_index_by_name(subject_name)
    }

    /// Asigna un profesor identificado por su id a la asignatura con el indice especificado.
    pub fn assign_teacher_to_subject(
        &mut self,
        subject_index: usize,
        teacher_id: u32,
    ) -> SimpleResult {
        AssignTeacherToSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .assign_teacher_to_subject(subject_index, teacher_id)
    }
}

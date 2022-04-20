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
    pub fn new(persistence: Box<dyn PersistenceTrait>) -> Self {
        let repository = create_repo(persistence);
        let repo_ref = Rc::new(RefCell::new(repository));
        let teachers_app = TeachersApp::new(&repo_ref);
        Self {
            repository: Rc::clone(&repo_ref),
            teachers_app,
        }
    }

    /// Load the subjects in the Model if needed
    pub fn load_subjects_if_needed(&mut self) -> () {
        self.repository.borrow_mut().load_subjects_if_needed();
    }

    /// Returns a copy of the subjects list
    pub fn get_subjects(&self) -> Result<Subjects, SimpleError> {
        let option = self.repository.borrow().model.subjects.clone();
        option.ok_or(SimpleError::new(&t!("couldnt_get_subjects")))
    }

    /// Add a new subject with the specified name
    pub fn add_new_subject(&mut self, name: &str) -> SimpleResult {
        AddSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .add_new_subject(name.to_string())
    }

    /// Remove the subject with the specified name
    pub fn remove_subject(&mut self, name: &str) -> SimpleResult {
        RemoveSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .remove_subject(name.to_string())
    }

    /// Returns the index of the subject with the specified name
    pub fn get_subject_index_by_name(
        &mut self,
        subject_name: &str,
    ) -> Result<usize, SimpleError> {
        GetSubjectIndexByNameUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .get_subject_index_by_name(subject_name)
    }

    /// Assign a teacher identified by his/her id to the subject with the specified index
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

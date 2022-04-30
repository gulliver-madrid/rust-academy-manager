use std::{cell::RefCell, rc::Rc};

use rust_i18n::t;

use crate::{
    domain::Subjects,
    errors::{SimpleError, SimpleResult},
    repository::Repository,
};

use super::usecases::{AddSubjectUseCase, RemoveSubjectUseCase, GetSubjectIndexByNameUseCase};

pub struct SubjectsApp {
    repository: Rc<RefCell<Repository>>,
}

impl SubjectsApp {
    pub fn new(repo_ref: &Rc<RefCell<Repository>>) -> Self {
        Self {
            repository: Rc::clone(repo_ref),
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

    /// Returns the index of the subject with the specified name
    pub fn get_subject_index_by_name(
        &self,
        subject_name: &str,
    ) -> Result<usize, SimpleError> {
        GetSubjectIndexByNameUseCase {
            repository: &self.repository.borrow(),
        }
        .get_subject_index_by_name(subject_name)
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
}

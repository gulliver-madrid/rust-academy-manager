use std::rc::Rc;

use crate::{
    domain::Subject,
    errors::{SimpleError, SimpleResult},
    repository::Repository,
};

use super::usecases::{
    AddSubjectUseCase, GetSubjectIndexByNameUseCase, RemoveSubjectUseCase,
};

pub struct SubjectsApp {
    repository: Rc<Repository>,
}

impl SubjectsApp {
    pub fn new(repo_ref: &Rc<Repository>) -> Self {
        Self {
            repository: Rc::clone(repo_ref),
        }
    }

    /// Load the subjects in the Model if needed
    pub fn load_subjects_if_needed(&self) -> () {
        self.repository.load_subjects_if_needed();
    }

    /// Returns a copy of the subjects list
    pub fn get_subjects_copy(&self) -> Result<Vec<Subject>, SimpleError> {
        let subjects = self
            .repository
            .model
            .borrow()
            .subjects
            .get_subjects_copy()?;
        Ok(subjects)
    }

    /// Returns the index of the subject with the specified name
    pub fn get_subject_index_by_name(
        &self,
        subject_name: &str,
    ) -> Result<usize, SimpleError> {
        GetSubjectIndexByNameUseCase {
            repository: Rc::clone(&self.repository),
        }
        .get_subject_index_by_name(subject_name)
    }

    /// Add a new subject with the specified name
    pub fn add_new_subject(&self, name: &str) -> SimpleResult {
        AddSubjectUseCase {
            repository: Rc::clone(&self.repository),
        }
        .add_new_subject(name.to_string())
    }

    /// Remove the subject with the specified name
    pub fn remove_subject(&self, name: &str) -> SimpleResult {
        RemoveSubjectUseCase {
            repository: Rc::clone(&self.repository),
        }
        .remove_subject(name.to_string())
    }
}

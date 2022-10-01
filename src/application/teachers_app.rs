use std::rc::Rc;

use crate::{domain::Teachers, errors::SimpleResult, repository::Repository};

use super::usecases::{AddTeacherUseCase, RemoveTeacherUseCase};

/// Allows interact with the repositories through predefined operations related to teachers
pub struct TeachersApp {
    repository: Rc<Repository>,
}

impl TeachersApp {
    pub fn new(repo_ref: &Rc<Repository>) -> Self {
        Self {
            repository: Rc::clone(repo_ref),
        }
    }

    /// Load teachers in the Model if needed
    pub fn load_teachers_if_needed(&self) -> () {
        self.repository.load_teachers_if_needed();
    }

    /// Returns a copy of the teachers list
    pub fn get_teachers(&self) -> Teachers {
        self.repository
            .model
            .borrow()
            .teachers
            .as_ref()
            .unwrap()
            .clone()
    }

    /// Add a new teacher with the specified name
    pub fn add_new_teacher(&self, name: &str) -> SimpleResult {
        AddTeacherUseCase {
            repository: Rc::clone(&self.repository),
        }
        .execute(name.to_string())
    }

    /// Remove the teacher with the specified name
    pub fn remove_teacher(&self, name: &str) -> SimpleResult {
        RemoveTeacherUseCase {
            repository: Rc::clone(&self.repository),
        }
        .remove_teacher(name.to_string())
    }
}

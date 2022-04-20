use std::{cell::RefCell, rc::Rc};

use crate::{domain::Teachers, errors::SimpleResult, repository::Repository};

use super::usecases::{AddTeacherUseCase, RemoveTeacherUseCase};

/// Allows interact with the repositories through predefined operations related to teachers
pub struct TeachersApp {
    repository: Rc<RefCell<Repository>>,
}


impl TeachersApp {
    pub fn new(repo_ref: &Rc<RefCell<Repository>>) -> Self {
        Self {
            repository: Rc::clone(repo_ref),
        }
    }

    /// Load teachers in the Model if needed
    pub fn load_teachers_if_needed(&mut self) -> () {
        self.repository.borrow_mut().load_teachers_if_needed();
    }

    /// Returns a copy of the teachers list
    pub fn get_teachers(&self) -> Teachers {
        self.repository
            .borrow()
            .model
            .teachers
            .as_ref()
            .unwrap()
            .clone()
    }

    /// Add a new teacher with the specified name
    pub fn add_new_teacher(&mut self, name: &str) -> SimpleResult {
        AddTeacherUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .add_new_teacher(name.to_string())
    }

    /// Remove the teacher with the specified name
    pub fn remove_teacher(&mut self, name: &str) -> SimpleResult {
        RemoveTeacherUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .remove_teacher(name.to_string())
    }
}

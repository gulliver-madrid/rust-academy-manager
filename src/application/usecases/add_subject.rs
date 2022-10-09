use std::rc::Rc;

use rust_i18n::t;

use crate::{
    domain::Subject,
    errors::{SimpleError, SimpleResult},
    repository::Repository,
    simple_error,
};

pub struct AddSubjectUseCase {
    pub repository: Rc<Repository>,
}

impl AddSubjectUseCase {
    pub fn add_new_subject(&self, name: String) -> SimpleResult {
        self.validate_name_is_free(&name)?;
        let next_id = self.get_next_id();
        let new_subject = create_new_subject(name, next_id);
        self.repository.add_subject(new_subject);
        Ok(())
    }
    fn validate_name_is_free(&self, name: &str) -> SimpleResult {
        let exists = self
            .repository
            .model
            .try_borrow()
            .unwrap()
            .subjects
            .exists_subject_with_name(name);
        if exists {
            return Err(create_already_exists_subject_error(name));
        }
        Ok(())
    }
    fn get_next_id(&self) -> u32 {
        let subjects = &self.repository.model.try_borrow().unwrap().subjects;
        match subjects.get_last_id() {
            Some(id) => id + 1,
            None => 1,
        }
    }
}

fn create_new_subject(name: String, id: u32) -> Subject {
    Subject {
        name,
        id,
        assigned_teachers: Vec::new(),
    }
}

fn create_already_exists_subject_error(name: &str) -> SimpleError {
    simple_error!("{} {}", t!("errors.already_exists_subject"), name)
}

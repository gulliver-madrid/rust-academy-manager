use std::rc::Rc;

use rust_i18n::t;

use crate::{
    domain::Teacher,
    errors::{SimpleError, SimpleResult},
    repository::Repository,
    simple_error,
};

pub struct AddTeacherUseCase {
    pub repository: Rc<Repository>,
}

impl AddTeacherUseCase {
    pub fn add_new_teacher(&mut self, name: String) -> SimpleResult {
        self.validate_teacher_doesnt_exist(&name)?;
        let id = self.get_next_id();
        let new_teacher = Self::create_new_teacher(name, id);
        self.repository.add_teacher(new_teacher);
        Ok(())
    }

    pub fn validate_teacher_doesnt_exist(&self, name: &str) -> SimpleResult {
        if self
            .repository
            .model
            .borrow()
            .does_teacher_exist_by_name(name)
        {
            return Self::create_already_exists_teacher_error(name);
        }
        Ok(())
    }

    fn create_new_teacher(name: String, id: u32) -> Teacher {
        Teacher {
            name,
            id,
            phone_number: String::new(),
        }
    }

    fn get_next_id(&self) -> u32 {
        self.repository.model.borrow().get_next_teacher_id()
    }

    fn create_already_exists_teacher_error(name: &str) -> SimpleResult {
        simple_error!("{} {}", t!("errors.already_exists_teacher"), name)
    }
}

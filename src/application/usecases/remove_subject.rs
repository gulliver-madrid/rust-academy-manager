use std::rc::Rc;

use rust_i18n::t;

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
    simple_error,
};

pub struct RemoveSubjectUseCase {
    pub repository: Rc<Repository>,
}

impl RemoveSubjectUseCase {
    pub fn remove_subject(&mut self, name: String) -> SimpleResult {
        self.repository
            .model
            .borrow_mut()
            .subjects
            .remove_by_name(name.to_owned())
            .ok_or_else(|| create_no_subject_with_this_name_error(&name))?;
        self.repository.save_subjects();
        Ok(())
    }
}
fn create_no_subject_with_this_name_error(name: &str) -> SimpleError {
    simple_error!("{} {}", t!("no_subject_with_name"), name)
}

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
        let model = Rc::clone(&self.repository.model);
        let result = model.borrow_mut().subjects.remove_by_name(name.to_owned());
        match result {
            Some(()) => {
                self.repository.save_subjects();
                Ok(())
            }
            None => simple_error!("{} {}", t!("no_subject_with_name"), name),
        }
    }
}

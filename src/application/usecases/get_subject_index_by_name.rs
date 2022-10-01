use std::rc::Rc;

use rust_i18n::t;

use crate::{errors::SimpleError, repository::Repository, simple_error};

pub struct GetSubjectIndexByNameUseCase {
    pub repository: Rc<Repository>,
}

impl GetSubjectIndexByNameUseCase {
    pub fn get_subject_index_by_name(
        &self,
        subject_name: &str,
    ) -> Result<usize, SimpleError> {
        let model = Rc::clone(&self.repository.model);
        let index = model.borrow().get_subject_index_by_name(subject_name);
        if let Some(index) = index {
            Ok(index)
        } else {
            Self::create_no_valid_name_error(subject_name)
        }
    }
    fn create_no_valid_name_error(subject_name: &str) -> Result<usize, SimpleError> {
        simple_error!("{}: {}", t!("no_valid_name"), subject_name)
    }
}

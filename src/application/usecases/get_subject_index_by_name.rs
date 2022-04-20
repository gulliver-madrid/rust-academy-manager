use rust_i18n::t;

use crate::{errors::SimpleError, repository::Repository};

pub struct GetSubjectIndexByNameUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl GetSubjectIndexByNameUseCase<'_> {
    pub fn get_subject_index_by_name(
        &mut self,
        subject_name: &str,
    ) -> Result<usize, SimpleError> {
        let subjects = self.repository.model.subjects.as_mut().unwrap();
        let index = subjects.iter().position(|a| a.name == subject_name);
        match index {
            Some(index) => Ok(index),
            None => Err(SimpleError::new(&format!(
                "{}: {}",
                t!("no_valid_name"),
                subject_name
            ))),
        }
    }
}

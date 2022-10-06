use std::rc::Rc;

use rust_i18n::t;

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
    simple_error,
};

pub struct AssignTeacherToSubjectUseCase {
    pub repository: Rc<Repository>,
}

impl AssignTeacherToSubjectUseCase {
    pub fn new(repo_ref: &Rc<Repository>) -> Self {
        Self {
            repository: Rc::clone(repo_ref),
        }
    }
    pub fn assign_teacher_to_subject(
        &self,
        subject_index: usize,
        teacher_id: u32,
    ) -> SimpleResult {
        let model = Rc::clone(&self.repository.model);
        let subjects_size = model.borrow().get_subjects_size();
        if subject_index >= subjects_size {
            panic!("{}", format!("Index error: {}", subject_index))
        }
        if !(self.repository.does_teacher_exist_by_id(teacher_id)) {
            return simple_error!("{} {}", t!("no_valid_id"), teacher_id);
        }
        model
            .borrow_mut()
            .assign_teacher_id_to_subject(subject_index, teacher_id)?;
        self.repository.save_subjects();
        Ok(())
    }
}

use std::rc::Rc;

use rust_i18n::t;

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
    simple_error,
};

pub struct RemoveTeacherUseCase {
    pub repository: Rc<Repository>,
}

impl RemoveTeacherUseCase {
    pub fn remove_teacher(&self, name: String) -> SimpleResult {
        self.repository.load_subjects_if_needed();
        let teacher_id = self
            .repository
            .model
            .borrow_mut()
            .remove_teacher(name.to_owned());
        let removed: u32;
        match teacher_id {
            Some(teacher_id) => removed = teacher_id,
            None => return create_teacher_doesnt_exist_error(&name),
        }

        self.remove_from_subjects_assignments(removed);
        self.repository.save_teachers();
        Ok(())
    }

    fn remove_from_subjects_assignments(&self, teacher_id: u32) {
        let model = Rc::clone(&self.repository.model);
        model
            .borrow_mut()
            .remove_teacher_from_subjects_assignments(teacher_id);
        self.repository.save_subjects();
    }
}

fn create_teacher_doesnt_exist_error(name: &str) -> SimpleResult {
    simple_error!("{}: {}", t!("no_teacher_with_name"), name)
}

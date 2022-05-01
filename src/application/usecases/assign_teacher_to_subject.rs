use rust_i18n::t;

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
    simple_error,
};

pub struct AssignTeacherToSubjectUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl AssignTeacherToSubjectUseCase<'_> {
    pub fn assign_teacher_to_subject(
        &mut self,
        subject_index: usize,
        teacher_id: u32,
    ) -> SimpleResult {
        let subjects = self.repository.model.subjects.as_mut().unwrap();

        let subject = subjects
            .get_mut(subject_index)
            .expect(&format!("Index error: {}", teacher_id));
        let teachers = self.repository.model.teachers.as_ref().unwrap();
        match teachers.iter().find(|teacher| teacher.id == teacher_id) {
            None => return simple_error!("{} {}", t!("no_valid_id"), teacher_id),
            _ => (),
        }
        subject.assigned_teachers.push(teacher_id);
        self.repository.persistence.save_subjects(subjects);
        Ok(())
    }
}

use crate::{errors::{SimpleResult, SimpleError}, repository::Repository};

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

        let subject = subjects.get_mut(subject_index).ok_or(SimpleError::new(
            &format!("{}", "Index error"),
        ))?;
        subject.assigned_teachers.push(teacher_id);
        self.repository.persistence.save_subjects(subjects);
        Ok(())
    }
}

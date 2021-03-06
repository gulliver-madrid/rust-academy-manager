use rust_i18n::t;

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
    simple_error,
};

pub struct RemoveTeacherUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl RemoveTeacherUseCase<'_> {
    pub fn remove_teacher(&mut self, name: String) -> SimpleResult {
        self.repository.load_subjects_if_needed();
        let teachers = self.repository.get_teachers_as_ref()?;
        let teacher_id: u32;
        let teacher_index: usize;
        match teachers.iter().position(|a| a.name == name) {
            Some(index) => {
                teacher_id = teachers[index].id;
                teacher_index = index;
            }
            None => return Self::create_teacher_doesnt_exist_error(&name),
        }
        self.remove_from_subjects_assignments(teacher_id);
        let teachers = &mut self.repository.model.teachers.as_mut().unwrap();
        teachers.remove(teacher_index);
        self.repository.persistence.save_teachers(teachers);
        Ok(())
    }

    fn remove_from_subjects_assignments(&mut self, teacher_id: u32) {
        let subjects = self.repository.model.subjects.as_mut().unwrap();
        for subject in &mut subjects.into_iter() {
            subject.assigned_teachers.retain(|id| *id != teacher_id);
        }
        self.repository.persistence.save_subjects(subjects);
    }
    fn create_teacher_doesnt_exist_error(name: &str) -> SimpleResult {
        simple_error!("{}: {}", t!("no_teacher_with_name"), name)
    }
}

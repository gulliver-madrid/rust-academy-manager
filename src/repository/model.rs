use std::{cell::RefCell, rc::Rc};

use crate::{
    domain::{Subject, Subjects, Teacher, Teachers},
    errors::SimpleResult,
};

pub fn create_model() -> Rc<RefCell<Model>> {
    Rc::new(RefCell::new(Model {
        teachers: Teachers::new(),
        subjects: Subjects::new(),
        _private: (),
    }))
}
pub struct Model {
    pub teachers: Teachers,
    pub subjects: Subjects,
    _private: (),
}

impl Model {
    pub fn add_subject(&mut self, subject: Subject) {
        self.subjects.add_subject(subject);
    }
    pub fn add_teacher(&mut self, teacher: Teacher) {
        self.teachers.add_teacher(teacher);
    }
    pub fn is_there_any_teacher_with_name(&self, name: &str) -> bool {
        self.teachers.any_meets(|teacher| teacher.name == name)
    }

    pub fn is_there_any_teacher_with_id(&self, id: u32) -> bool {
        self.teachers.any_meets(|teacher| teacher.id == id)
    }

    pub fn get_next_teacher_id(&self) -> u32 {
        self.teachers.get_last_id().map(|id| id + 1).unwrap_or(1)
    }

    pub fn assign_teacher_id_to_subject(
        &mut self,
        subject_index: usize,
        teacher_id: u32,
    ) -> SimpleResult {
        {
            self.subjects
                .assign_teacher_id_to_subject(subject_index, teacher_id)
        }
    }

    pub fn get_subject_index_by_name(&self, subject_name: &str) -> Option<usize> {
        self.subjects.get_subject_index_by_name(subject_name)
    }

    pub fn remove_teacher_from_subjects_assignments(&mut self, teacher_id: u32) {
        self.subjects
            .remove_teacher_from_subjects_assignments(teacher_id)
    }

    pub fn load_subjects(&mut self, subjects: Vec<Subject>) {
        self.subjects.load_data(subjects);
    }

    pub fn load_teachers(&mut self, teachers: Vec<Teacher>) {
        self.teachers.load_data(teachers)
    }
}

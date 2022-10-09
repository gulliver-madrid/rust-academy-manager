use std::{cell::RefCell, rc::Rc};

use crate::{
    domain::{Subject, Subjects, Teacher, Teachers},
    errors::SimpleResult,
    helpers,
};

pub const TEACHERS_SHOULD_BE_DEFINED: &'static str = "Teachers should be defined";

pub fn create_model() -> Rc<RefCell<Model>> {
    Rc::new(RefCell::new(Model {
        teachers: None,
        subjects: Subjects::new(),
        _private: (),
    }))
}
pub struct Model {
    pub teachers: Option<Teachers>,
    pub subjects: Subjects,
    _private: (),
}

impl Model {
    pub fn remove_subject(&mut self, name: String) -> Option<()> {
        self.subjects.remove_subject(name)
    }
    /// Delete the teacher with the given name
    /// Return an Option wrapping the teacher id
    pub fn remove_teacher(&mut self, name: String) -> Option<u32> {
        let teachers = &mut self.teachers.as_mut().unwrap();
        teachers.iter().position(|a| a.name == name).map(|index| {
            let id = teachers[index].id;
            teachers.remove(index);
            id
        })
    }
    pub fn add_subject(&mut self, subject: Subject) {
        self.subjects.add_subject(subject);
    }
    pub fn add_teacher(&mut self, teacher: Teacher) {
        self.teachers.as_mut().unwrap().push(teacher);
    }
    pub fn does_teacher_exist_by_name(&self, name: &str) -> bool {
        self.does_teacher_meet_condition(|teacher| teacher.name == name)
    }

    pub fn does_teacher_exist_by_id(&self, id: u32) -> bool {
        self.does_teacher_meet_condition(|teacher| teacher.id == id)
    }

    pub fn get_next_teacher_id(&self) -> u32 {
        let teachers = self.teachers.as_ref().unwrap();
        match helpers::get_last_element(teachers) {
            Some(last_teacher) => last_teacher.id + 1,
            None => 1,
        }
    }

    pub fn get_subjects_size(&self) -> usize {
        self.subjects.get_subjects_size()
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
        self.subjects.load_subjects(subjects);
    }

    pub fn load_teachers(&mut self, teachers: Teachers) {
        self.teachers = Some(teachers);
    }

    pub fn does_teacher_meet_condition<F: Fn(&&Teacher) -> bool>(
        &self,
        condition: F,
    ) -> bool {
        self.teachers
            .as_ref()
            .expect(TEACHERS_SHOULD_BE_DEFINED)
            .iter()
            .find(condition)
            .is_some()
    }
}

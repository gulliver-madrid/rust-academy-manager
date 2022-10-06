use std::{cell::RefCell, rc::Rc};

use crate::{
    domain::{Subject, Subjects, Teacher, Teachers},
    helpers,
};

const SUBJECTS_SHOULD_BE_DEFINED: &'static str = "Subjects should be defined";

pub fn create_model() -> Rc<RefCell<Model>> {
    Rc::new(RefCell::new(Model {
        teachers: None,
        subjects: None,
        _private: (),
    }))
}

pub struct Model {
    pub teachers: Option<Teachers>,
    pub subjects: Option<Subjects>,
    _private: (),
}

impl Model {
    pub fn remove_subject(&mut self, name: String) -> Option<()> {
        let subjects = &mut self.subjects.as_mut().unwrap();
        match subjects.iter().position(|a| a.name == name) {
            Some(index) => {
                subjects.remove(index);
                Some(())
            }
            None => None,
        }
    }
    pub fn remove_teacher(&mut self, name: String) -> Option<u32> {
        let teachers = &mut self.teachers.as_mut().unwrap();
        match teachers.iter().position(|a| a.name == name) {
            Some(index) => {
                let id = teachers[index].id;
                teachers.remove(index);
                Some(id)
            }
            None => None,
        }
    }
    pub fn add_subject(&mut self, subject: Subject) {
        let subjects = self.subjects.as_mut().unwrap();
        subjects.push(subject);
    }
    pub fn add_teacher(&mut self, teacher: Teacher) {
        let teachers = self.teachers.as_mut().unwrap();
        teachers.push(teacher);
    }
    pub fn does_teacher_exist_by_name(&self, name: &str) -> bool {
        match self
            .teachers
            .as_ref()
            .unwrap()
            .iter()
            .find(|teacher| teacher.name == name)
        {
            Some(_) => true,
            None => false,
        }
    }
    pub fn does_teacher_exist_by_id(&self, id: u32) -> bool {
        match self
            .teachers
            .as_ref()
            .unwrap()
            .iter()
            .find(|teacher| teacher.id == id)
        {
            Some(_) => true,
            None => false,
        }
    }
    pub fn get_next_teacher_id(&self) -> u32 {
        let teachers = self.teachers.as_ref().unwrap();
        if let Some(last_teacher) = helpers::get_last_element(&teachers) {
            last_teacher.id + 1
        } else {
            1
        }
    }

    pub fn get_subjects_size(&self) -> usize {
        self.subjects.as_ref().unwrap().len()
    }
    pub fn assign_teacher_id_to_subject(
        &mut self,
        subject_index: usize,
        teacher_id: u32,
    ) {
        self.subjects.as_mut().unwrap()[subject_index]
            .assigned_teachers
            .push(teacher_id);
    }

    pub fn get_subject_index_by_name(&self, subject_name: &str) -> Option<usize> {
        self.subjects
            .as_ref()
            .unwrap()
            .iter()
            .position(|a| a.name == subject_name)
    }

    pub fn remove_teacher_from_subjects_assignments(&mut self, teacher_id: u32) {
        for subject in self
            .subjects
            .as_mut()
            .expect(SUBJECTS_SHOULD_BE_DEFINED)
            .iter_mut()
        {
            subject.assigned_teachers.retain(|id| *id != teacher_id);
        }
    }

    pub fn load_subjects(&mut self, subjects: Subjects) {
        self.subjects = Some(subjects);
    }

    pub fn load_teachers(&mut self, teachers: Teachers) {
        self.teachers = Some(teachers);
    }
}

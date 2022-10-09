use rust_i18n::t;

use crate::{
    errors::{SimpleError, SimpleResult},
    helpers,
};

use super::Subject;

pub struct Subjects(Option<Vec<Subject>>);

pub const SUBJECTS_SHOULD_BE_DEFINED: &'static str = "Subjects should be defined";

impl Subjects {
    pub fn new() -> Self {
        Subjects(None)
    }
    pub fn exists_subject_with_name(&self, name: &str) -> bool {
        let vec: &Vec<Subject> = self.0.as_ref().unwrap();
        vec.iter().find(|subject| subject.name == name).is_some()
    }
    pub fn get_vec_copy(&self) -> Result<Vec<Subject>, SimpleError> {
        let subjects = self
            .0
            .as_ref()
            .ok_or(SimpleError::new(&t!("couldnt_get_subjects")))?
            .clone();
        Ok(subjects)
    }
    pub fn get_last_id(&self) -> Option<u32> {
        let vec = self.0.as_ref().unwrap();
        helpers::get_last_element(&vec).map(|subject| subject.id)
    }
    pub fn remove_by_name(&mut self, name: String) -> Option<()> {
        let subjects = &mut self.0.as_mut()?;
        match subjects.iter().position(|a| a.name == name) {
            Some(index) => {
                subjects.remove(index);
                Some(())
            }
            None => None,
        }
    }
    pub fn add_subject(&mut self, subject: Subject) {
        self.0.as_mut().unwrap().push(subject);
    }

    pub fn assign_teacher_id_to_subject(
        &mut self,
        subject_index: usize,
        teacher_id: u32,
    ) -> SimpleResult {
        {
            self.0
                .as_mut()
                .ok_or_else(|| format!("{}", SUBJECTS_SHOULD_BE_DEFINED))?
                .get_mut(subject_index)
                .ok_or_else(|| format!("Wrong subject index: {subject_index}"))?
                .assigned_teachers
                .push(teacher_id);
            Ok(())
        }
    }
    pub fn get_subject_index_by_name(&self, subject_name: &str) -> Option<usize> {
        self.0
            .as_ref()
            .unwrap()
            .iter()
            .position(|a| a.name == subject_name)
    }
    pub fn remove_teacher_from_subjects_assignments(&mut self, teacher_id: u32) {
        for subject in self
            .0
            .as_mut()
            .expect(SUBJECTS_SHOULD_BE_DEFINED)
            .iter_mut()
        {
            subject.assigned_teachers.retain(|id| *id != teacher_id);
        }
    }
    pub fn load_data(&mut self, subjects: Vec<Subject>) {
        self.0 = Some(subjects);
    }
    pub fn are_loaded(&self) -> bool {
        self.0.is_some()
    }
    pub fn get_vec(&self) -> &Vec<Subject> {
        return self.0.as_ref().unwrap();
    }
}

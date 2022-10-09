use crate::{errors::SimpleError, helpers};

use super::Teacher;

pub struct Teachers(Option<Vec<Teacher>>);

pub const TEACHERS_SHOULD_BE_DEFINED: &'static str = "Teachers should be defined";

impl Teachers {
    pub fn new() -> Self {
        Teachers(None)
    }
    pub fn get_last_id(&self) -> Option<u32> {
        let vec = self.0.as_ref().unwrap();
        helpers::get_last_element(&vec).map(|teacher| teacher.id)
    }
    pub fn add_teacher(&mut self, teacher: Teacher) {
        self.0.as_mut().unwrap().push(teacher);
    }
    pub fn remove_by_name(&mut self, name: String) -> Option<u32> {
        let teachers = &mut self.0.as_mut()?;
        teachers.iter().position(|a| a.name == name).map(|index| {
            let id = teachers[index].id;
            teachers.remove(index);
            id
        })
    }
    pub fn get_vec(&self) -> &Vec<Teacher> {
        return self.0.as_ref().unwrap();
    }
    /// Returns a copy of the teachers list
    pub fn get_vec_copy(&self) -> Result<Vec<Teacher>, SimpleError> {
        let teachers = self
            .0
            .as_ref()
            .ok_or(SimpleError::new(TEACHERS_SHOULD_BE_DEFINED))?
            .clone();
        Ok(teachers)
    }
    pub fn load_data(&mut self, teachers: Vec<Teacher>) {
        self.0 = Some(teachers);
    }
    pub fn any_meets<F: Fn(&Teacher) -> bool>(&self, condition: F) -> bool {
        self.0
            .as_ref()
            .expect(TEACHERS_SHOULD_BE_DEFINED)
            .iter()
            .any(condition)
    }
    pub fn are_loaded(&self) -> bool {
        self.0.is_some()
    }
    #[cfg(test)]
    pub fn get_number_of_teachers(&self) -> usize {
        self.0.as_ref().unwrap().len()
    }
}

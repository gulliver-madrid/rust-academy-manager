use rust_i18n::t;

use crate::{
    domain::{Teacher, Teachers},
    errors::{SimpleError, SimpleResult},
    helpers,
    repository::Repository,
    simple_error,
};

pub struct AddTeacherUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl AddTeacherUseCase<'_> {
    pub fn add_new_teacher(&mut self, name: String) -> SimpleResult {
        let teachers = self.repository.model.teachers.as_ref().unwrap();
        Self::validate_teacher_doesnt_exist(teachers, &name)?;
        let id = Self::get_next_id(teachers);
        let new_teacher = Self::create_new_teacher(name, id);
        self.add_teacher(new_teacher);
        Ok(())
    }

    fn validate_teacher_doesnt_exist(teachers: &Teachers, name: &str) -> SimpleResult {
        match teachers.iter().find(|teacher| teacher.name == name) {
            Some(_) => Self::create_already_exists_teacher_error(name),
            None => Ok(()),
        }
    }

    fn create_new_teacher(name: String, id: u32) -> Teacher {
        Teacher {
            name,
            id,
            phone_number: String::new(),
        }
    }

    fn get_next_id(teachers: &Teachers) -> u32 {
        if let Some(last_teacher) = helpers::get_last_element(teachers) {
            last_teacher.id + 1
        } else {
            1
        }
    }

    fn add_teacher(&mut self, teacher: Teacher) {
        let teachers = &mut self.repository.model.teachers.as_mut().unwrap();
        teachers.push(teacher);
        self.repository.persistence.save_teachers(teachers);
    }

    fn create_already_exists_teacher_error(name: &str) -> SimpleResult {
        simple_error!("{} {}", t!("errors.already_exists_teacher"), name)
    }
}

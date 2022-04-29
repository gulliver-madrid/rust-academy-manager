use rust_i18n::t;

use crate::{
    domain::{Teacher, Teachers},
    errors::{SimpleError, SimpleResult},
    helpers,
    repository::Repository,
};

pub struct AddTeacherUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl AddTeacherUseCase<'_> {
    pub fn add_new_teacher(&mut self, name: String) -> SimpleResult {
        let teachers = self.repository.model.teachers.as_ref().unwrap();
        self.validate_teacher_doesnt_exist(teachers, &name)?;
        let new_teacher = self.create_new_teacher(name);
        self.add_teacher(new_teacher);
        Ok(())
    }

    fn validate_teacher_doesnt_exist(&self, teachers: &Teachers, name: &str) -> SimpleResult {
        for teacher in teachers {
            if teacher.name == *name {
                return Self::create_already_exists_teacher_error(name);
            }
        };
        Ok(())
    }
    
    fn create_new_teacher(&self, name: String) -> Teacher {
        Teacher {
            name,
            id: self.get_next_id(),
            phone_number: String::new(),
        }
    }

    fn get_next_id(&self) -> u32 {
        let teachers = &self.repository.model.teachers.as_ref().unwrap();
        let last_teacher =
            helpers::get_last_element(teachers).expect(&t!("errors.no_teacher"));
        last_teacher.id + 1
    }


    fn add_teacher(&mut self, teacher: Teacher) {
        let teachers = &mut self.repository.model.teachers.as_mut().unwrap();
        teachers.push(teacher);
        self.repository.persistence.save_teachers(teachers);
    }

    fn create_already_exists_teacher_error(name: &str) -> SimpleResult {
        Err(SimpleError::new(&format!(
            "{} {}",
            t!("errors.already_exists_teacher"),
            name
        )))
    }
}



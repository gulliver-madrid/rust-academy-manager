use rust_i18n::t;

use crate::{
    domain::Teacher,
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
        for teacher in teachers {
            if teacher.name == name {
                return Err(SimpleError::new(&format!(
                    "{} {}",
                    t!("already_exists_teacher"),
                    name
                )));
            }
        }
        let next_id: u32 = self.get_next_id();
        let new_teacher = self.create_new_teacher(name, next_id);
        self.add_teacher(new_teacher);
        Ok(())
    }

    fn get_next_id(&self) -> u32 {
        let teachers = &self.repository.model.teachers.as_ref().unwrap();
        let last_teacher =
            helpers::get_last_element(teachers).expect(&t!("errors.no_teacher"));
        last_teacher.id + 1
    }

    fn create_new_teacher(&self, name: String, id: u32) -> Teacher {
        Teacher {
            name,
            id,
            phone_number: String::new(),
        }
    }

    fn add_teacher(&mut self, teacher: Teacher) {
        let teachers = &mut self.repository.model.teachers.as_mut().unwrap();
        teachers.push(teacher);
        self.repository.persistence.save_teachers(teachers);
    }
}

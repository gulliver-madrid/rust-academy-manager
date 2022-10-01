use std::rc::Rc;

use rust_i18n::t;

use crate::{
    domain::{Subject, Subjects},
    errors::{SimpleError, SimpleResult},
    helpers,
    repository::Repository,
    simple_error,
};

pub struct AddSubjectUseCase {
    pub repository: Rc<Repository>,
}

impl AddSubjectUseCase {
    pub fn add_new_subject(&self, name: String) -> SimpleResult {
        let new_subject: Subject;
        {
            let model = self.repository.model.borrow();
            let subjects = model.subjects.as_ref().unwrap();
            validate_name_is_free(subjects, &name)?;
            let next_id: u32 = get_next_id(subjects);
            new_subject = create_new_subject(name, next_id);
        }
        self.repository.add_subject(new_subject);
        Ok(())
    }
}

fn validate_name_is_free(subjects: &Subjects, name: &str) -> SimpleResult {
    match subjects.iter().find(|subject| subject.name == name) {
        Some(_) => create_already_exists_subject_error(name),
        None => Ok(()),
    }
}
fn create_new_subject(name: String, id: u32) -> Subject {
    Subject {
        name,
        id,
        assigned_teachers: Vec::new(),
    }
}

fn get_next_id(subjects: &Subjects) -> u32 {
    if let Some(last_subject) = helpers::get_last_element(subjects) {
        last_subject.id + 1
    } else {
        1
    }
}
fn create_already_exists_subject_error(name: &str) -> SimpleResult {
    simple_error!("{} {}", t!("errors.already_exists_subject"), name)
}

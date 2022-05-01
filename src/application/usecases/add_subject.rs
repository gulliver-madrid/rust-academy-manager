use rust_i18n::t;

use crate::{
    domain::{Subject, Subjects},
    errors::{SimpleError, SimpleResult},
    helpers,
    repository::Repository,
    simple_error,
};

pub struct AddSubjectUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl AddSubjectUseCase<'_> {
    pub fn add_new_subject(&mut self, name: String) -> SimpleResult {
        let subjects = self.repository.model.subjects.as_ref().unwrap();
        validate_name_is_free(subjects, &name)?;
        let next_id: u32 = get_next_id(subjects);
        let new_subject = create_new_subject(name, next_id);
        self.add_subject(new_subject);
        Ok(())
    }

    fn add_subject(&mut self, subject: Subject) {
        let subjects = &mut self.repository.model.subjects.as_mut().unwrap();
        subjects.push(subject);
        self.repository.persistence.save_subjects(subjects);
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
    let last_subject =
        helpers::get_last_element(subjects).expect(&t!("errors.no_subject"));
    last_subject.id + 1
}
fn create_already_exists_subject_error(name: &str) -> SimpleResult {
    simple_error!("{} {}", t!("errors.already_exists_subject"), name)
}

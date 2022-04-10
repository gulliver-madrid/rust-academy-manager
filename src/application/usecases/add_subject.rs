use crate::{
    domain::{Subject, Subjects},
    errors::{SimpleError, SimpleResult},
    helpers,
    repository::Repository,
    texts,
};

pub struct AddSubjectUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl AddSubjectUseCase<'_> {
    pub fn add_new_subject(&mut self, name: String) -> SimpleResult {
        let subjects = self.repository.model.subjects.as_ref().unwrap();
        self.validate_name_is_free(subjects, &name)?;
        let next_id: u32 = self.get_next_id();
        let new_subject = self.create_new_subject(name, next_id);
        self.add_subject(new_subject);
        Ok(())
    }

    fn validate_name_is_free(
        &self,
        subjects: &Subjects,
        name: &str,
    ) -> SimpleResult {
        for subject in subjects {
            if subject.name == name {
                return Err(SimpleError::new(&format!(
                    "Ya existe una asignatura llamada {}",
                    name
                )));
            }
        }
        Ok(())
    }

    fn get_next_id(&self) -> u32 {
        let subjects = &self.repository.model.subjects.as_ref().unwrap();
        let last_subject = helpers::get_last_element(subjects)
            .expect(texts::errors::NO_SUBJECT);
        last_subject.id + 1
    }

    fn create_new_subject(&self, name: String, id: u32) -> Subject {
        Subject {
            name,
            id,
            assigned_teachers: Vec::new(),
        }
    }

    fn add_subject(&mut self, subject: Subject) {
        let subjects = &mut self.repository.model.subjects.as_mut().unwrap();
        subjects.push(subject);
        self.repository.persistence.save_subjects(subjects);
    }
}

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
};

pub struct RemoveSubjectUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl RemoveSubjectUseCase<'_> {
    pub fn remove_subject(&mut self, name: String) -> SimpleResult {
        let subjects = &mut self.repository.model.subjects.as_mut().unwrap();
        match subjects.iter().position(|a| a.name == name) {
            Some(index) => {
                subjects.remove(index);
                self.repository.persistence.save_subjects(subjects);
                Ok(())
            }
            None => Err(SimpleError::new(&format!(
                "No hay ninguna asignatura con el nombre {}",
                name
            ))),
        }
    }
}

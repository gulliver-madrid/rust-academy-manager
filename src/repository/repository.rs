use rust_i18n::t;

use crate::{domain::Teacher, errors::SimpleError};

use super::{model::Model, PersistenceTrait};

pub fn create_repo(persistencia: Box<dyn PersistenceTrait>) -> Repository {
    let repository = Repository {
        persistence: persistencia,
        model: Model {
            teachers: None,
            subjects: None,
        },
    };
    repository
}
pub struct Repository {
    pub persistence: Box<dyn PersistenceTrait>,
    pub model: Model,
}

impl Repository {
    pub fn load_teachers_if_needed(&mut self) {
        match self.model.teachers {
            None => self.populate_teachers(),
            _ => {}
        }
    }

    pub fn load_subjects_if_needed(&mut self) {
        match self.model.subjects {
            None => self.populate_subjects(),
            _ => {}
        }
    }
    pub fn get_teachers_as_ref(&self) -> Result<&Vec<Teacher>, SimpleError> {
        let result = self.model.teachers.as_ref();
        match result {
            Some(profesores) => Ok(profesores),
            None => Err(SimpleError::new(
                &t!("couldnt_access_teachers_list"),
            )),
        }
    }
    fn populate_subjects(&mut self) {
        self.model.subjects = Some(self.persistence.load_subjects());
    }
    fn populate_teachers(&mut self) {
        self.model.teachers = Some(self.persistence.load_teachers());
    }
}

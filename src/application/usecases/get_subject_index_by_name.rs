use crate::{errors::SimpleError, repository::Repository};

pub struct GetSubjectIndexByNameUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl GetSubjectIndexByNameUseCase<'_> {
    pub fn get_subject_index_by_name(
        &mut self,
        subject_name: &str,
    ) -> Result<usize, SimpleError> {
        let asignaturas = self.repository.model.subjects.as_mut().unwrap();
        let busqueda_index =
            asignaturas.iter().position(|a| a.name == subject_name);
        match busqueda_index {
            Some(index) => Ok(index),
            None => Err(SimpleError::new(&format!(
                "Nombre no válido: {}",
                subject_name
            ))),
        }
    }
}

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
};

pub struct RemoveSubjectUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl RemoveSubjectUseCase<'_> {
    pub fn remove_subject(&mut self, nombre: String) -> SimpleResult {
        let subjects = &mut self.repository.modelo.asignaturas.as_mut().unwrap();
        match subjects.iter().position(|a| a.nombre == nombre) {
            Some(index) => {
                subjects.remove(index);
                self.repository
                    .persistencia
                    .save_asignaturas(subjects);
                Ok(())
            }
            None => Err(SimpleError::new(&format!(
                "No hay ninguna asignatura con el nombre {}",
                nombre
            ))),
        }
    }
}

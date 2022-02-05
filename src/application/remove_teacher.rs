use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
};

pub struct RemoveTeacherUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl RemoveTeacherUseCase<'_> {
    pub fn eliminar_profesor(&mut self, nombre: String) -> SimpleResult {
        let profesores = &mut self.repository.modelo.profesores.as_mut().unwrap();
        match profesores.iter().position(|a| a.nombre == nombre) {
            Some(index) => {
                profesores.remove(index);
                self.repository.persistencia.save_profesores(profesores);
                Ok(())
            }
            None => Err(SimpleError::new(&format!(
                "No hay ningún profesor con el nombre {}",
                nombre
            ))),
        }
    }
}

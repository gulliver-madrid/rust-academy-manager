use crate::errors::SimpleError;

use super::{modelo::Modelo, persistencia::Persistencia, SimpleResult};

pub struct RemoveTeacherUseCase<'a> {
    pub persistencia: &'a mut Persistencia,
    pub modelo: &'a mut Modelo,
}

impl RemoveTeacherUseCase<'_> {
    pub fn eliminar_profesor(&mut self, nombre: String) -> SimpleResult {
        let profesores = &mut self.modelo.profesores.as_mut().unwrap();
        match profesores.iter().position(|a| a.nombre == nombre) {
            Some(index) => {
                profesores.remove(index);
                self.persistencia.save_profesores(profesores);
                Ok(())
            }
            None => Err(SimpleError::new(&format!(
                "No hay ningún profesor con el nombre {}",
                nombre
            ))),
        }
    }
}

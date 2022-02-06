use crate::{dominio::Profesor, errors::SimpleError};

use super::{modelo::Modelo, Persistencia};

pub struct Repository {
    pub persistencia: Persistencia,
    pub modelo: Modelo,
}

impl Repository {
    pub fn load_profesores(&mut self) {
        match self.modelo.profesores {
            None => {
                self.modelo.profesores = Some(self.persistencia.load_profesores());
            }
            _ => {}
        }
    }

    pub fn load_subjects(&mut self) {
        match self.modelo.asignaturas {
            None => {
                self.modelo.asignaturas = Some(self.persistencia.load_subjects());
            }
            _ => {}
        }
    }
    pub fn get_profesores_as_mut(
        &mut self,
    ) -> Result<&mut Vec<Profesor>, SimpleError> {
        let result = self.modelo.profesores.as_mut();
        match result {
            Some(profesores) => Ok(profesores),
            None => Err(SimpleError::new(
                "No se pudo acceder al listado de profesores",
            )),
        }
    }
}

use crate::{dominio::Profesor, errors::SimpleError};

use super::{modelo::Modelo, PersistenciaTrait};

pub fn create_repo(persistencia: Box<dyn PersistenciaTrait>) -> Repository {
    let repository = Repository {
        persistencia,
        modelo: Modelo {
            profesores: None,
            asignaturas: None,
        },
    };
    repository
}
pub struct Repository {
    pub persistencia: Box<dyn PersistenciaTrait>,
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
    pub fn get_profesores_as_ref(&self) -> Result<&Vec<Profesor>, SimpleError> {
        let result = self.modelo.profesores.as_ref();
        match result {
            Some(profesores) => Ok(profesores),
            None => Err(SimpleError::new(
                "No se pudo acceder al listado de profesores",
            )),
        }
    }
}

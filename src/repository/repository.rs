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
}

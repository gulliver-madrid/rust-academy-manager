use crate::{
    dominio::{teachers::Profesores, Profesor},
    helpers,
    persistencia::Persistencia,
    textos,
};

pub struct Modelo {
    pub profesores: Option<Profesores>,
}

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
            Some(_) => {}
        }
    }
    pub fn anadir_nuevo_profesor(&mut self, nombre: String) {
        let next_id: u32 = self._get_next_id();
        let nuevo_profesor = self._crear_nuevo_profesor(nombre, next_id);
        self._anadir_profesor(nuevo_profesor);
    }

    fn _anadir_profesor(&mut self, profesor: Profesor) {
        let profesores = &mut self.modelo.profesores.as_mut().unwrap();
        profesores.push(profesor);
        self.persistencia.save_profesores(profesores);
    }

    fn _get_next_id(&self) -> u32 {
        let profesores = &self.modelo.profesores.as_ref().unwrap();
        let last_profesor =
            helpers::get_last_element(profesores).expect(textos::errores::NO_PROFESOR);
        last_profesor.id + 1
    }

    fn _crear_nuevo_profesor(&self, nombre: String, id: u32) -> Profesor {
        Profesor {
            nombre,
            id,
            telefono: String::new(),
        }
    }
}

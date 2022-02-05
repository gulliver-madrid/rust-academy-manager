use super::modelo::Modelo;
use super::persistencia::Persistencia;
use crate::{dominio::Profesor, helpers, textos};

pub struct AddTeacherUseCase<'a> {
    pub persistencia: &'a mut Persistencia,
    pub modelo: &'a mut Modelo,
}

impl AddTeacherUseCase<'_> {
    pub fn anadir_nuevo_profesor(&mut self, nombre: String) {
        let next_id: u32 = self.get_next_id();
        let nuevo_profesor = self.create_new_teacher(nombre, next_id);
        self.add_teacher(nuevo_profesor);
    }

    fn get_next_id(&self) -> u32 {
        let profesores = &self.modelo.profesores.as_ref().unwrap();
        let last_profesor =
            helpers::get_last_element(profesores).expect(textos::errores::NO_PROFESOR);
        last_profesor.id + 1
    }

    fn create_new_teacher(&self, nombre: String, id: u32) -> Profesor {
        Profesor {
            nombre,
            id,
            telefono: String::new(),
        }
    }

    fn add_teacher(&mut self, profesor: Profesor) {
        let profesores = &mut self.modelo.profesores.as_mut().unwrap();
        profesores.push(profesor);
        self.persistencia.save_profesores(profesores);
    }
}

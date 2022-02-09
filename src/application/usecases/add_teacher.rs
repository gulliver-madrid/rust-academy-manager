use crate::{
    dominio::Profesor, errors::{SimpleResult, SimpleError}, helpers, repository::Repository, textos,
};

pub struct AddTeacherUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl AddTeacherUseCase<'_> {
    pub fn anadir_nuevo_profesor(&mut self, nombre: String) -> SimpleResult {
        let teachers = self.repository.modelo.profesores.as_ref().unwrap();
        for teacher in teachers{
            if teacher.nombre == nombre{
                return Err(SimpleError::new(&format!(
                    "Ya existe un profesor con el nombre {}",
                    nombre
                )));
            }
        }
        let next_id: u32 = self.get_next_id();
        let nuevo_profesor = self.create_new_teacher(nombre, next_id);
        self.add_teacher(nuevo_profesor);
        Ok(())
    }

    fn get_next_id(&self) -> u32 {
        let profesores = &self.repository.modelo.profesores.as_ref().unwrap();
        let last_profesor = helpers::get_last_element(profesores)
            .expect(textos::errores::NO_PROFESOR);
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
        let profesores = &mut self.repository.modelo.profesores.as_mut().unwrap();
        profesores.push(profesor);
        self.repository.persistencia.save_profesores(profesores);
    }
}

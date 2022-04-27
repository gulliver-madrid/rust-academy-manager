use crate::{
    dominio::Asignatura, errors::SimpleResult, helpers, repository::Repository,
    textos,
};

pub struct AddSubjectUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl AddSubjectUseCase<'_> {
    pub fn add_new_subject(&mut self, nombre: String) -> SimpleResult {
        let next_id: u32 = self.get_next_id();
        let new_subject = self.create_new_subject(nombre, next_id);
        self.add_subject(new_subject);
        Ok(())
    }

    fn get_next_id(&self) -> u32 {
        let subjects = &self.repository.modelo.asignaturas.as_ref().unwrap();
        let last_subject = helpers::get_last_element(subjects)
            .expect(textos::errores::NO_ASIGNATURA);
        last_subject.id + 1
    }

    fn create_new_subject(&self, nombre: String, id: u32) -> Asignatura {
        Asignatura {
            nombre,
            id,
            profesores_asignados: Vec::new(),
        }
    }

    fn add_subject(&mut self, subject: Asignatura) {
        let subjects = &mut self.repository.modelo.asignaturas.as_mut().unwrap();
        subjects.push(subject);
        self.repository.persistencia.save_asignaturas(subjects);
    }
}

use crate::{
    errors::{SimpleError, SimpleResult},
    repository::Repository,
};

pub struct RemoveTeacherUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl RemoveTeacherUseCase<'_> {
    pub fn eliminar_profesor(&mut self, nombre: String) -> SimpleResult {
        self.repository.load_subjects();
        let profesores = self.repository.get_profesores_as_mut()?;
        let id_profesor: u32;
        let index_profesor: usize;
        match profesores.iter().position(|a| a.nombre == nombre) {
            Some(index) => {
                id_profesor = profesores[index].id;
                index_profesor = index;
            }
            None => {
                return Err(SimpleError::new(&format!(
                    "No hay ningún profesor con el nombre {}",
                    nombre
                )));
            }
        }
        self.remove_from_subjects_assignments(id_profesor);
        let profesores = &mut self.repository.modelo.profesores.as_mut().unwrap();
        profesores.remove(index_profesor);
        self.repository.persistencia.save_profesores(profesores);
        Ok(())
    }

    fn remove_from_subjects_assignments(&mut self, id_profesor: u32) {
        let asignaturas = self.repository.modelo.asignaturas.as_mut().unwrap();
        for asignatura in asignaturas {
            match asignatura
                .profesores_asignados
                .iter()
                .position(|id| *id == id_profesor)
            {
                Some(index) => {
                    asignatura.profesores_asignados.remove(index);
                }
                _ => (),
            }
        }
        let asignaturas = self.repository.modelo.asignaturas.as_mut().unwrap();
        self.repository.persistencia.save_asignaturas(asignaturas);
    }
}

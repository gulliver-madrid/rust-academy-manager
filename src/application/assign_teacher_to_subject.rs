use crate::{repository::Repository, errors::SimpleResult};

pub struct AssignTeacherToSubjectUseCase<'a> {
    pub repository: &'a mut Repository,
}

impl AssignTeacherToSubjectUseCase<'_>{
    pub fn asignar_profesor_a_asignatura(
        &mut self,
        index_asignatura: usize,
        id_profesor: u32,
    ) -> SimpleResult {
        let asignaturas = self.repository.modelo.asignaturas.as_mut().unwrap();

        let asignatura = &mut asignaturas[index_asignatura];

        asignatura.profesores_asignados.push(id_profesor);
        self.repository.persistencia.save_asignaturas(asignaturas);
        Ok(())
    }
}
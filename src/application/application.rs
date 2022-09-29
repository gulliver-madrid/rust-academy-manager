use std::{cell::RefCell, rc::Rc};

use crate::{errors::SimpleResult, repository::Repository};

use super::{
    subjects_app::SubjectsApp, teachers_app::TeachersApp,
    usecases::AssignTeacherToSubjectUseCase,
};

pub struct Application {
    repository: Rc<RefCell<Repository>>,
    pub teachers_app: TeachersApp,
    pub subjects_app: SubjectsApp,
}

impl Application {
    pub fn new(
        repository: Rc<RefCell<Repository>>,
        teachers_app: TeachersApp,
        subjects_app: SubjectsApp,
    ) -> Self {
        Self {
            repository,
            teachers_app,
            subjects_app,
        }
    }
    /// Assign a teacher identified by his/her id to the subject with the specified index
    pub fn assign_teacher_to_subject(
        &mut self,
        subject_index: usize,
        teacher_id: u32,
    ) -> SimpleResult {
        AssignTeacherToSubjectUseCase {
            repository: &mut self.repository.borrow_mut(),
        }
        .assign_teacher_to_subject(subject_index, teacher_id)
    }
}

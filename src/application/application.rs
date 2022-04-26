use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::SimpleResult,
    repository::{create_repo, PersistenceTrait, Repository},
};

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
    pub fn new(persistence: Box<dyn PersistenceTrait>) -> Self {
        let repository = create_repo(persistence);
        let repo_ref = Rc::new(RefCell::new(repository));
        let teachers_app = TeachersApp::new(&repo_ref);
        let subjects_app = SubjectsApp::new(&repo_ref);
        Self {
            repository: Rc::clone(&repo_ref),
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

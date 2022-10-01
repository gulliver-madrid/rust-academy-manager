use std::{cell::RefCell, rc::Rc};

use super::application::Application;
use super::usecases::AssignTeacherToSubjectUseCase;
use super::{subjects_app::SubjectsApp, teachers_app::TeachersApp};
use crate::repository::{create_repo, PersistenceTrait};

pub fn create_application(persistence: Box<dyn PersistenceTrait>) -> Application {
    let repository = create_repo(persistence);
    let repo_ref = Rc::new(repository);
    let teachers_app = RefCell::new(TeachersApp::new(&repo_ref));
    let subjects_app = RefCell::new(SubjectsApp::new(&repo_ref));
    let assign_teacher_to_subject_usecase =
        RefCell::new(AssignTeacherToSubjectUseCase::new(&repo_ref));

    Application::new(
        assign_teacher_to_subject_usecase,
        teachers_app,
        subjects_app,
    )
}

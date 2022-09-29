use std::{cell::RefCell, rc::Rc};

use super::application::Application;
use super::{subjects_app::SubjectsApp, teachers_app::TeachersApp};
use crate::repository::{create_repo, PersistenceTrait};

pub fn create_application(persistence: Box<dyn PersistenceTrait>) -> Application {
    let repository = create_repo(persistence);
    let repo_ref = Rc::new(RefCell::new(repository));
    let teachers_app = TeachersApp::new(&repo_ref);
    let subjects_app = SubjectsApp::new(&repo_ref);
    let repo = Rc::clone(&repo_ref);
    Application::new(repo, teachers_app, subjects_app)
}

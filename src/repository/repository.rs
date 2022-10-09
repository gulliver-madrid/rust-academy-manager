use std::{cell::RefCell, rc::Rc};

use crate::domain::{Subject, Teacher};

use super::{
    model::{create_model, Model},
    PersistenceTrait,
};

pub fn create_repository(persistence: Rc<dyn PersistenceTrait>) -> Repository {
    create_repo_providing_model(persistence, create_model())
}

pub fn create_repo_providing_model(
    persistence: Rc<dyn PersistenceTrait>,
    model: Rc<RefCell<Model>>,
) -> Repository {
    let repository = Repository { persistence, model };
    repository
}
pub struct Repository {
    pub persistence: Rc<dyn PersistenceTrait>,
    pub model: Rc<RefCell<Model>>,
}

impl Repository {
    pub fn load_teachers_if_needed(&self) {
        let are_loaded = match self.model.borrow().teachers {
            None => false,
            _ => true,
        };
        if !are_loaded {
            self.populate_teachers()
        }
    }
    pub fn load_subjects_if_needed(&self) {
        if !self.model.borrow().subjects.are_loaded() {
            self.populate_subjects()
        }
    }

    pub fn add_subject(&self, subject: Subject) {
        self.model.borrow_mut().add_subject(subject);
        self.save_subjects();
    }
    pub fn save_subjects(&self) {
        let model = self.model.borrow();
        let subjects = model.subjects.get();
        self.persistence.save_subjects(&subjects);
    }
    pub fn save_teachers(&self) {
        let model = self.model.borrow();
        let teachers = model.teachers.as_ref().unwrap();
        self.persistence.save_teachers(&teachers);
    }

    pub fn add_teacher(&self, teacher: Teacher) {
        self.model.borrow_mut().add_teacher(teacher);
        self.save_teachers();
    }

    pub fn does_teacher_exist_by_id(&self, id: u32) -> bool {
        self.model.borrow().does_teacher_exist_by_id(id)
    }

    fn populate_subjects(&self) {
        let subjects = self.persistence.load_subjects();
        self.model.borrow_mut().load_subjects(subjects)
    }
    fn populate_teachers(&self) {
        let teachers = self.persistence.load_teachers();
        self.model.borrow_mut().load_teachers(teachers)
    }
}

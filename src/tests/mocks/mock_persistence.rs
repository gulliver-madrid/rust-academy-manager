#![cfg(test)]

use std::cell::RefCell;

use crate::{
    domain::{Subject, Teacher, Teachers},
    repository::PersistenceTrait,
};

pub struct MockPersistence {
    pub mock_teachers: RefCell<Vec<Teacher>>,
    pub mock_subjects: RefCell<Vec<Subject>>,
}

impl PersistenceTrait for MockPersistence {
    fn save_teachers(&self, teachers: &Teachers) {
        self.mock_teachers.replace(teachers.to_owned());
    }
    fn save_subjects(&self, subjects: &Vec<Subject>) {
        self.mock_subjects.replace(subjects.to_owned());
    }
    fn load_teachers(&self) -> Teachers {
        return self.mock_teachers.borrow().clone();
    }
    fn load_subjects(&self) -> Vec<Subject> {
        return self.mock_subjects.borrow().clone();
    }
}

pub fn create_void_mock_persistence() -> MockPersistence {
    MockPersistence {
        mock_teachers: RefCell::new(Vec::<Teacher>::new()),
        mock_subjects: RefCell::new(Vec::<Subject>::new()),
    }
}

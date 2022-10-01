#![cfg(test)]

use crate::{
    domain::{Subject, Subjects, Teacher, Teachers},
    repository::PersistenceTrait,
};

pub struct MockPersistence {
    pub mock_teachers: Vec<Teacher>,
    pub mock_subjects: Vec<Subject>,
}

impl PersistenceTrait for MockPersistence {
    fn save_teachers(&self, _teachers: &Teachers) {}
    fn save_subjects(&self, _subjects: &Subjects) {}
    fn load_teachers(&self) -> Teachers {
        return self.mock_teachers.clone();
    }
    fn load_subjects(&self) -> Subjects {
        return self.mock_subjects.clone();
    }
}

pub fn create_void_mock_persistence() -> MockPersistence {
    MockPersistence {
        mock_teachers: Vec::<Teacher>::new(),
        mock_subjects: Vec::<Subject>::new(),
    }
}

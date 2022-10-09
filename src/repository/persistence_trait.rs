use crate::domain::{Subject, Teachers};

pub trait PersistenceTrait {
    fn save_teachers(&self, teachers: &Teachers);
    fn save_subjects(&self, subjects: &Vec<Subject>);
    fn load_teachers(&self) -> Teachers;
    fn load_subjects(&self) -> Vec<Subject>;
}

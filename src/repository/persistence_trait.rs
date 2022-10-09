use crate::domain::{Subject, Teacher};

pub trait PersistenceTrait {
    fn save_teachers(&self, teachers: &Vec<Teacher>);
    fn save_subjects(&self, subjects: &Vec<Subject>);
    fn load_teachers(&self) -> Vec<Teacher>;
    fn load_subjects(&self) -> Vec<Subject>;
}

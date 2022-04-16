use crate::domain::{Teachers, Subjects};

pub trait PersistenceTrait {
    fn save_teachers(&self, teachers: &Teachers);
    fn save_subjects(&self, subjects: &Subjects);
    fn load_teachers(&self) -> Teachers;
    fn load_subjects(&self) -> Subjects;
}
use crate::domain::{Teachers, Subjects};

pub struct Model {
    pub teachers: Option<Teachers>,
    pub subjects: Option<Subjects>,
}
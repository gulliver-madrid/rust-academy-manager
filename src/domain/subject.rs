use serde::{Deserialize, Serialize};

pub type Subjects = Vec<Subject>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Subject {
    pub name: String,
    pub id: u32,
    pub assigned_teachers: Vec<u32>,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SerializableTeacher {
    pub name: String,
    pub id: u32,
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SerializableSubject {
    pub name: String,
    pub id: u32,
    #[serde(default = "default_assigned_teachers")]
    pub assigned_teachers: Vec<u32>,
}

fn default_assigned_teachers() -> Vec<u32> {
    Vec::new()
}
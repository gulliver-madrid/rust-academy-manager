use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SerializableTeacher {
    pub name: String,
    pub id: u32,
    pub phone_number: String,
}
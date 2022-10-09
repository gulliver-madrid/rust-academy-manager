use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subject {
    pub name: String,
    pub id: u32,
    pub assigned_teachers: Vec<u32>,
}

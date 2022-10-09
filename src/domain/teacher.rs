use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Teacher {
    pub name: String,
    pub id: u32,
    pub phone_number: String,
}

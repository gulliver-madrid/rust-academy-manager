#[derive(Clone)]
pub struct Profesor {
    pub nombre: String,
    pub id: u32,
    pub telefono: String,
}

pub type Profesores = Vec<Profesor>;
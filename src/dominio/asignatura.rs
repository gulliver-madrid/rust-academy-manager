#[derive(Clone)]
pub struct Asignatura {
    pub nombre: String,
    pub id: u32,
}

pub type Asignaturas = Vec<Asignatura>;

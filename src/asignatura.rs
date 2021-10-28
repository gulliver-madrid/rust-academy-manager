pub type Asignaturas = Vec<Asignatura>;

#[derive(Clone)]
pub struct Asignatura {
    pub nombre: String,
    pub id: u32,
}

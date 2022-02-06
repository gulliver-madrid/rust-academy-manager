use crate::dominio::{Profesores, Asignaturas};

pub struct Modelo {
    pub profesores: Option<Profesores>,
    pub asignaturas: Option<Asignaturas>,
}
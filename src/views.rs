use crate::dominio::Asignatura;
use crate::dominio::Profesor;
use crate::helpers::set_number_chars;

pub trait View {
    fn crear_linea_tabla(&self) -> String;
}
impl View for Profesor {
    fn crear_linea_tabla(&self) -> String {
        let profesor_str = set_number_chars(&self.nombre, 22);
        let tlf_str = match self.telefono.as_str() {
            "" => "desconocido",
            otro => otro,
        };
        format!(
            "Nombre: {}  Id: {}  Telefono: {}",
            profesor_str,
            format!("{:0>3}", self.id),
            tlf_str
        )
    }
}
impl View for Asignatura {
    fn crear_linea_tabla(&self) -> String {
        let asignatura_str = set_number_chars(&self.nombre, 28);

        format!(
            "Asignatura: {}  Id: {}",
            asignatura_str,
            format!("{:0>3}", self.id)
        )
    }
}

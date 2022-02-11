use crate::{components::Control};

pub struct MenuAsignarProfesor<'a> {
    pub control: &'a mut Control,
}

impl MenuAsignarProfesor<'_> {
    pub fn abrir_menu(&mut self) {
        let consola = &self.control.consola;
        consola.mostrar("Elige la asignatura a la que quieras asignar profesor");
        let nombre_asignatura: String;
        match consola.pide_texto_a_usuario() {
            Some(entrada) => nombre_asignatura = entrada,
            None => {
                consola.mostrar("Operación cancelada");
                consola.pausa_enter("continuar");
                return;
            }
        }
        let index_asignatura: usize;
        match self
            .control
            .application
            .get_subject_index_by_name(&nombre_asignatura)
        {
            Ok(index) => {
                index_asignatura = index;
            }
            Err(e) => {
                consola.mostrar(&e.to_string());
                consola.pausa_enter("continuar");
                return;
            }
        }
        consola.mostrar(&format!("Asignatura introducida: {}", nombre_asignatura));
        consola.mostrar("Introduce el ID del profesor");
        if let Some(entrada) = consola.pide_texto_a_usuario() {
            let id_profesor = entrada.parse::<u32>().unwrap();
            let result = self
                .control
                .application
                .asignar_profesor_a_asignatura(index_asignatura, id_profesor);
            if result.is_ok() {
                consola.mostrar("Profesor asignado correctamente");
            } else {
                consola.mostrar("No se pudo realizar la operación");
            }
            consola.pausa_enter("continuar");
        }
    }
}

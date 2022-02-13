use crate::{components::Control};

pub struct MenuAsignarProfesor<'a> {
    pub control: &'a mut Control,
}

impl MenuAsignarProfesor<'_> {
    pub fn abrir_menu(&mut self) {
        let ui = &self.control.ui;
        ui.mostrar("Elige la asignatura a la que quieras asignar profesor");
        let nombre_asignatura: String;
        match ui.pide_texto_a_usuario() {
            Some(entrada) => nombre_asignatura = entrada,
            None => {
                ui.mostrar("Operación cancelada");
                ui.pausa_enter("continuar");
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
                ui.mostrar(&e.to_string());
                ui.pausa_enter("continuar");
                return;
            }
        }
        ui.mostrar(&format!("Asignatura introducida: {}", nombre_asignatura));
        ui.mostrar("Introduce el ID del profesor");
        if let Some(entrada) = ui.pide_texto_a_usuario() {
            let id_profesor = entrada.parse::<u32>().unwrap();
            let result = self
                .control
                .application
                .asignar_profesor_a_asignatura(index_asignatura, id_profesor);
            if result.is_ok() {
                ui.mostrar("Profesor asignado correctamente");
            } else {
                ui.mostrar("No se pudo realizar la operación");
            }
            ui.pausa_enter("continuar");
        }
    }
}

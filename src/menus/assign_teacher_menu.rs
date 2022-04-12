use rust_i18n::t;

use crate::{components::Control};

pub struct AssignTeacherMenu<'a> {
    pub control: &'a mut Control,
}

impl AssignTeacherMenu<'_> {
    pub fn open_menu(&mut self) {
        let ui = &self.control.ui;
        ui.show("Elige la asignatura a la que quieras asignar profesor");
        let subject_name: String;
        match ui.ask_text_to_user() {
            Some(entered_text) => subject_name = entered_text,
            None => {
                ui.show("Operación cancelada");
                ui.pause_enter(&t!("continue"));
                return;
            }
        }
        let subject_index: usize;
        match self
            .control
            .application
            .get_subject_index_by_name(&subject_name)
        {
            Ok(index) => {
                subject_index = index;
            }
            Err(e) => {
                ui.show(&e.to_string());
                ui.pause_enter(&t!("continue"));
                return;
            }
        }
        ui.show(&format!("Asignatura introducida: {}", subject_name));
        ui.show("Introduce el ID del profesor");
        if let Some(entered_text) = ui.ask_text_to_user() {
            let teacher_id = entered_text.parse::<u32>().unwrap();
            let result = self
                .control
                .application
                .assign_teacher_to_subject(subject_index, teacher_id);
            if result.is_ok() {
                ui.show("Profesor asignado correctamente");
            } else {
                ui.show("No se pudo realizar la operación");
            }
            ui.pause_enter(&t!("continue"));
        }
    }
}

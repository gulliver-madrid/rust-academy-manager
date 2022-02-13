use crate::{components::Control, errors::SimpleResult, textos, ui::UserInterface};

pub struct MenuEliminarAsignatura<'a> {
    pub control: &'a mut Control,
}

impl MenuEliminarAsignatura<'_> {
    pub fn abrir_menu(&mut self) {
        let ui = &self.control.ui;
        self.mostrar_texto_menu(ui);
        match ui.pide_texto_a_usuario() {
            None => (),
            Some(nombre) => {
                let result = self.control.application.remove_subject(&nombre);
                let msg = self.get_info_result(result, nombre);
                ui.mostrar(&msg);
                ui.pausa_enter("continuar");
            }
        }
    }

    fn mostrar_texto_menu(&self, ui: &UserInterface) {
        ui.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA_A_ELIMINAR);
    }

    fn get_info_result(&self, result: SimpleResult, nombre: String) -> String {
        match result {
            Ok(_) => format!("Se eliminó exitosamente la asignatura {}", nombre),
            Err(e) => e.to_string(),
        }
    }
}

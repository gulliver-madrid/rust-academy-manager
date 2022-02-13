use crate::{components::Control, errors::SimpleResult, textos, ui::UserInterface};

pub struct MenuEliminarProfesor<'a> {
    pub control: &'a mut Control,
}

impl MenuEliminarProfesor<'_> {
    pub fn abrir_menu(&mut self) {
        let ui = &self.control.ui;
        self.mostrar_texto_menu(&ui);

        if let Some(nombre) = ui.pide_texto_a_usuario() {
            let result = self
                .control
                .application
                .teachers_app
                .eliminar_profesor(&nombre);
            let msg = self.get_info_result(result, nombre);
            ui.mostrar(&msg);
            ui.pausa_enter("continuar");
        };
    }

    fn mostrar_texto_menu(&self, ui: &UserInterface) {
        ui.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR_A_ELIMINAR);
    }

    fn get_info_result(&self, result: SimpleResult, nombre: String) -> String {
        match result {
            Ok(_) => format!("Se eliminó exitosamente al profesor {}", nombre),
            Err(e) => e.to_string(),
        }
    }
}

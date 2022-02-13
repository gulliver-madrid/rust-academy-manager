use crate::{components::Control, textos, ui::UserInterface};

pub struct MenuAnadirProfesor<'a> {
    pub control: &'a mut Control,
}

impl MenuAnadirProfesor<'_> {
    pub fn abrir_menu(&mut self) {
        let ui = &self.control.ui;
        self.mostrar_texto_menu(&ui);
        if let Some(nombre) = ui.pide_texto_a_usuario() {
            let result = self
                .control
                .application
                .teachers_app
                .anadir_nuevo_profesor(&nombre);
            let msg = match result {
                Ok(_) => format!("Profesor con nombre {} añadido", nombre),
                Err(e) => e.to_string(),
            };
            ui.mostrar(&msg);
            ui.pausa_enter("continuar");
        }
    }

    fn mostrar_texto_menu(&self, ui: &UserInterface) {
        ui.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
    }
}

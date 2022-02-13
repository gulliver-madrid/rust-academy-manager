use crate::{components::Control, textos, ui::UserInterface};

pub struct MenuAnadirAsignatura<'a> {
    pub control: &'a mut Control,
}

impl MenuAnadirAsignatura<'_> {
    pub fn abrir_menu(&mut self) {
        let ui = &self.control.ui;
        self.mostrar_texto_menu(ui);
        if let Some(nombre) = ui.pide_texto_a_usuario() {
            let result = self.control.application.add_new_subject(&nombre);
            let msg = match result {
                Ok(_) => format!("Asignatura {} añadida", nombre),
                Err(e) => e.to_string(),
            };
            ui.mostrar(&msg);
            ui.pausa_enter("continuar");
        }
    }

    fn mostrar_texto_menu(&self, ui: &UserInterface) {
        ui.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA);
    }
}

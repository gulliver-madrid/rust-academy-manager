use crate::{components::Control, consola::Consola, errors::SimpleResult, textos};

pub struct MenuEliminarProfesor<'a> {
    pub control: &'a mut Control,
}

impl MenuEliminarProfesor<'_> {
    pub fn abrir_menu(&mut self) {
        let consola = &self.control.consola;
        self.mostrar_texto_menu(&consola);

        if let Some(nombre) = consola.pide_texto_a_usuario() {
            let result = self
                .control
                .application
                .teachers_app
                .eliminar_profesor(&nombre);
            let msg = self.get_info_result(result, nombre);
            consola.mostrar(&msg);
            consola.pausa_enter("continuar");
        };
    }

    fn mostrar_texto_menu(&self, consola: &Consola) {
        consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR_A_ELIMINAR);
    }

    fn get_info_result(&self, result: SimpleResult, nombre: String) -> String {
        match result {
            Ok(_) => format!("Se eliminó exitosamente al profesor {}", nombre),
            Err(e) => e.to_string(),
        }
    }
}

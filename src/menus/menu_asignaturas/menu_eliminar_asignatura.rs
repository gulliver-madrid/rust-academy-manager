use crate::{components::Control, consola::Consola, errors::SimpleResult, textos};

pub struct MenuEliminarAsignatura<'a> {
    pub control: &'a mut Control,
}

impl MenuEliminarAsignatura<'_> {
    pub fn abrir_menu(&mut self) {
        let consola = &self.control.consola;
        self.mostrar_texto_menu(consola);
        match consola.pide_texto_a_usuario() {
            None => (),
            Some(nombre) => {
                let result = self.control.application.remove_subject(&nombre);
                let msg = self.get_info_result(result, nombre);
                consola.mostrar(&msg);
                consola.pausa_enter("continuar");
            }
        }
    }

    fn mostrar_texto_menu(&self, consola: &Consola) {
        consola.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA_A_ELIMINAR);
    }

    fn get_info_result(&self, result: SimpleResult, nombre: String) -> String {
        match result {
            Ok(_) => format!("Se eliminó exitosamente la asignatura {}", nombre),
            Err(e) => e.to_string(),
        }
    }
}

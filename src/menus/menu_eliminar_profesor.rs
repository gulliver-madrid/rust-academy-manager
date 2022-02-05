use crate::{components::Control, consola::Consola, repository::SimpleResult, textos};

use super::Menu;

pub struct MenuEliminarProfesor {}

impl Menu for MenuEliminarProfesor {
    fn abrir_menu(&mut self, control: &mut Control) {
        self.open_remove_teacher_menu(control);
    }
}

impl MenuEliminarProfesor {
    fn open_remove_teacher_menu(&mut self, control: &mut Control) {
        let consola = &control.consola;
        self.mostrar_texto_menu(&consola);

        if let Some(nombre) = consola.pide_texto_a_usuario() {
            let result = control.application.eliminar_profesor(&nombre);
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

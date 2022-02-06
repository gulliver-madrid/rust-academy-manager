use crate::{
    components::Control, consola::Consola, 
    errors::SimpleResult, menus::Menu, textos,
};

pub struct MenuEliminarAsignatura {}

impl Menu for MenuEliminarAsignatura {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

impl MenuEliminarAsignatura {
    fn _abrir_menu(&mut self, control: &mut Control) {
        let consola = &control.consola;
        self.mostrar_texto_menu(consola);
        match control.consola.pide_texto_a_usuario() {
            None => (),
            Some(nombre) => {
                let result = control.application.remove_subject(&nombre);
                let msg = self.get_info_result(result, nombre);
                consola.mostrar(&msg);
                control.consola.pausa_enter("continuar");
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

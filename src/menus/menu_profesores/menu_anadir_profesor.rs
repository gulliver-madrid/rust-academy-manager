use crate::{components::Control, consola::Consola, menus::Menu, textos};

pub struct MenuAnadirProfesor {}

impl MenuAnadirProfesor {
    fn _abrir_menu(&mut self, control: &mut Control) {
        let consola = &control.consola;
        self.mostrar_texto_menu(&consola);
        if let Some(nombre) = control.consola.pide_texto_a_usuario() {
            let result = control.application.teachers_app.anadir_nuevo_profesor(&nombre);
            if result.is_ok() {
                consola.mostrar(&format!("Profesor con nombre {} añadido", nombre));
                consola.pausa_enter("continuar");
            }
        }
    }

    fn mostrar_texto_menu(&self, consola: &Consola) {
        consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
    }
}

impl Menu for MenuAnadirProfesor {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

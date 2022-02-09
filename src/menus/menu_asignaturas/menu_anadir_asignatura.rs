use crate::{components::Control, consola::Consola, menus::Menu, textos};

pub struct MenuAnadirAsignatura {}

impl MenuAnadirAsignatura {
    fn _abrir_menu(&mut self, control: &mut Control) {
        let consola = &control.consola;
        self.mostrar_texto_menu(consola);
        if let Some(nombre) = control.consola.pide_texto_a_usuario() {
            let result = control.application.add_new_subject(&nombre);
            let msg = match result {
                Ok(_) => format!("Asignatura {} añadida", nombre),
                Err(e) => e.to_string(),
            };
            consola.mostrar(&msg);
            consola.pausa_enter("continuar");
        }
    }

    fn mostrar_texto_menu(&self, consola: &Consola) {
        consola.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA);
    }
}

impl Menu for MenuAnadirAsignatura {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

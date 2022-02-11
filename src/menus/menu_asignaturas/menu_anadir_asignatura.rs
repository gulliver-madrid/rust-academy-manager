use crate::{components::Control, consola::Consola,  textos};

pub struct MenuAnadirAsignatura <'a> {
    pub control: &'a mut Control,
}

impl MenuAnadirAsignatura<'_> {
    pub fn abrir_menu(&mut self) {
        let consola = &self.control.consola;
        self.mostrar_texto_menu(consola);
        if let Some(nombre) = consola.pide_texto_a_usuario() {
            let result = self.control.application.add_new_subject(&nombre);
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



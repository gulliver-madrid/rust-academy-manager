use crate::{components::Control, consola::Consola, textos};

pub struct MenuAnadirProfesor<'a> {
    pub control: &'a mut Control,
}

impl MenuAnadirProfesor<'_> {

    pub fn abrir_menu(&mut self) {
        let consola = &self.control.consola;
        self.mostrar_texto_menu(&consola);
        if let Some(nombre) = consola.pide_texto_a_usuario() {
            let result = self
                .control
                .application
                .teachers_app
                .anadir_nuevo_profesor(&nombre);
            let msg = match result {
                Ok(_) => format!("Profesor con nombre {} añadido", nombre),
                Err(e) => e.to_string(),
            };
            consola.mostrar(&msg);
            consola.pausa_enter("continuar");
        }
    }

    fn mostrar_texto_menu(&self, consola: &Consola) {
        consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
    }
}

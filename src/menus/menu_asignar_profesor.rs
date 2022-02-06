use crate::{components::Control, consola::Consola, dominio::Asignatura};

use super::Menu;

pub struct MenuAsignarProfesor {
    pub index_asignatura: usize,
}

impl MenuAsignarProfesor {
    fn _abrir_menu(&mut self, control: &mut Control) {
        let consola = &control.consola;
        
        let asignaturas = &mut control
            .application
            .repository
            .modelo
            .asignaturas
            .as_mut()
            .unwrap();

        let asignatura: &mut Asignatura= &mut asignaturas[self.index_asignatura];
        self.mostrar_texto_menu(asignatura, consola);
        if let Some(entrada) = consola.pide_texto_a_usuario() {
            let id_profesor = entrada.parse::<u32>().unwrap();
            asignatura.profesores_asignados.push(id_profesor);
            control
                .application
                .repository
                .persistencia
                .save_asignaturas(asignaturas);
            consola.mostrar("Profesor asignado correctamente");
            consola.pausa_enter("continuar");
        }
    }

    fn mostrar_texto_menu(&self, asignatura: &Asignatura, consola: &Consola) {
        consola.mostrar(&format!("Asignatura: {}", asignatura.nombre));
        consola.mostrar("Añade un profesor por su id");
    }
}

impl Menu for MenuAsignarProfesor {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

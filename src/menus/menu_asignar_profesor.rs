use crate::{
    components::Control,
    dominio::asignatura::{Asignatura, Asignaturas},
};

use super::Menu;

pub struct MenuAsignarProfesor<'a> {
    pub asignaturas: &'a mut Asignaturas,
    pub id_asignatura: u32,
}

impl MenuAsignarProfesor<'_> {
    fn _abrir_menu(&mut self, control: &mut Control) {
        let asignatura: &Asignatura;
        let index: usize;
        match self
            .asignaturas
            .iter()
            .position(|a| a.id == self.id_asignatura)
        {
            Some(_index) => {
                index = _index;
                asignatura = &self.asignaturas[index];
            }
            None => panic!(),
        }

        self.mostrar_texto_menu(asignatura, control);
        match control.consola.pide_texto_a_usuario() {
            None => (),
            Some(entrada) => {
                let id_profesor = entrada.parse::<u32>().unwrap();
                let asignatura = &mut self.asignaturas[index];
                asignatura.profesores_asignados.push(id_profesor);
                control
                    .application
                    .repository
                    .persistencia
                    .save_asignaturas(self.asignaturas);
                control.consola.mostrar("Profesor asignado correctamente");
                control.consola.pausa_enter("continuar");
            }
        }
    }

    fn mostrar_texto_menu(&self, asignatura: &Asignatura, control: &mut Control) {
        control
            .consola
            .mostrar(&format!("asignatura: {}", asignatura.nombre));
        control.consola.mostrar("Añade un profesor por su id");
    }
}

impl Menu for MenuAsignarProfesor<'_> {
    fn abrir_menu(&mut self, control: &mut Control) {
        self._abrir_menu(control);
    }
}

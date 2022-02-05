use crate::{components::Control, dominio::asignatura::Asignaturas, textos};

use super::Menu;

pub struct MenuEliminarAsignatura<'a> {
    asignaturas: &'a mut Asignaturas,
}

impl MenuEliminarAsignatura<'_> {
    pub fn new<'a>(asignaturas: &'a mut Asignaturas) -> MenuEliminarAsignatura<'a> {
        MenuEliminarAsignatura { asignaturas }
    }

    fn _abrir_menu(&mut self, control: &Control) {
        self.mostrar_texto_menu(control);
        match control.consola.pide_texto_a_usuario() {
            None => (),
            Some(nombre) => {
                self._eliminar_asignatura(nombre, control);
                control.consola.pausa_enter("continuar");
            }
        }
    }

    fn mostrar_texto_menu(&self, control: &Control) {
        control
            .consola
            .mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA_A_ELIMINAR);
    }

    fn _eliminar_asignatura(&mut self, nombre: String, control: &Control) {
        match self.asignaturas.iter().position(|a| a.nombre == nombre) {
            Some(index) => {
                self.asignaturas.remove(index);
                control
                    .repository
                    .persistencia
                    .save_asignaturas(&self.asignaturas);
            }
            None => control.consola.mostrar(&format!(
                "No hay ninguna asignatura con el nombre {}",
                nombre
            )),
        }
    }
}

impl Menu for MenuEliminarAsignatura<'_> {
    fn abrir_menu(&mut self, control: &Control) {
        self._abrir_menu(control);
    }
}

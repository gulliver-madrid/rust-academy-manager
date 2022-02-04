use crate::{
    components::Control,
    dominio::{asignatura::Asignaturas, Asignatura},
    helpers, textos,
};

use super::Menu;

pub struct MenuAnadirAsignatura<'a> {
    asignaturas: &'a mut Asignaturas,
}

impl MenuAnadirAsignatura<'_> {
    pub fn new<'a>(asignaturas: &'a mut Asignaturas) -> MenuAnadirAsignatura<'a> {
        MenuAnadirAsignatura { asignaturas }
    }

    fn _abrir_menu(&mut self, control: &Control) {
        let next_id: u32 = self.get_next_id();
        self.mostrar_texto_menu(control);
        match control.consola.pide_texto_a_usuario() {
            None => return,
            Some(nombre) => {
                self._anadir_nueva_asignatura(nombre, next_id, control);
            }
        }
    }

    fn mostrar_texto_menu(&self, control: &Control) {
        control.consola.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA);
    }

    fn _anadir_nueva_asignatura(&mut self, nombre: String, id: u32, control: &Control) {
        let nueva = Asignatura { nombre, id };
        self.asignaturas.push(nueva);
        control.repository.save_asignaturas(&self.asignaturas);
    }

    fn get_next_id(&self) -> u32 {
        let last_profe = helpers::get_last_element(&self.asignaturas)
            .expect(textos::errores::NO_ASIGNATURA);
        last_profe.id + 1
    }
}

impl Menu for MenuAnadirAsignatura<'_> {
    fn abrir_menu(&mut self, control: &Control) {
        self._abrir_menu(control);
    }
}

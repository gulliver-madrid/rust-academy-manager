use crate::{
    consola::Consola,
    dominio::{asignatura::Asignaturas, Asignatura},
    helpers, repo, textos,
};

use super::Menu;

pub struct MenuAnadirAsignatura<'a> {
    _consola: &'a Consola,
    asignaturas: &'a mut Asignaturas,
}

impl MenuAnadirAsignatura<'_> {
    pub fn new<'a>(
        consola: &'a Consola,
        asignaturas: &'a mut Asignaturas,
    ) -> MenuAnadirAsignatura<'a> {
        MenuAnadirAsignatura {
            _consola: consola,
            asignaturas,
        }
    }

    fn _abrir_menu(&mut self) {
        let next_id: u32 = self.get_next_id();
        self.mostrar_texto_menu();
        match self._consola.pide_texto_a_usuario() {
            None => return,
            Some(nombre) => {
                self.anadir_nueva_asignatura(nombre, next_id);
            }
        }
    }

    fn mostrar_texto_menu(&self) {
        self._consola.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA);
    }

    fn anadir_nueva_asignatura(&mut self, nombre: String, id: u32) {
        let nueva = Asignatura { nombre, id };
        self.asignaturas.push(nueva);
        repo::save_asignaturas(self.asignaturas.clone());
    }

    fn get_next_id(&self) -> u32 {
        let last_profe = helpers::get_last_element(&self.asignaturas)
            .expect(textos::errores::NO_ASIGNATURA);
        last_profe.id + 1
    }
}

impl Menu for MenuAnadirAsignatura<'_> {
    fn abrir_menu(&mut self) {
        self._abrir_menu();
    }
}

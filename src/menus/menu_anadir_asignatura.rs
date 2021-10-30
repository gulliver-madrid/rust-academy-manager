use crate::{
    consola,
    dominio::{asignatura::Asignaturas, Asignatura},
    helpers, repo, textos,
};

use super::Menu;

pub struct MenuAnadirAsignatura<'a> {
    _consola: &'a consola::Consola,
    asignaturas: &'a mut Asignaturas,
}
impl MenuAnadirAsignatura<'_> {
    pub fn new<'a>(
        consola: &'a consola::Consola,
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
                let nueva = self.crear_nueva_asignatura(nombre, next_id);
                self.anadir_asignatura(nueva);
            }
        }
    }

    fn mostrar_texto_menu(&self) {
        self._consola.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA);
    }

    fn anadir_asignatura(&mut self, asignatura: Asignatura) {
        self.asignaturas.push(asignatura);
        repo::save_asignaturas(self.asignaturas.clone());
    }

    fn get_next_id(&self) -> u32 {
        let last_profe = helpers::get_last_element(&self.asignaturas) //
            .expect(textos::ERROR_NO_ASIGNATURA);
        last_profe.id + 1
    }

    fn crear_nueva_asignatura(&self, nombre: String, id: u32) -> Asignatura {
        Asignatura { nombre, id }
    }
}

impl Menu for MenuAnadirAsignatura<'_> {
    fn abrir_menu(&mut self) {
        self._abrir_menu();
    }
}

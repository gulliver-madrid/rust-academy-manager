use crate::{
    components::Control,
    dominio::{teachers::Profesores, Profesor},
    helpers, textos,
};

use super::Menu;

pub struct MenuAnadirProfesor<'a> {
    profesores: &'a mut Profesores,
}
impl MenuAnadirProfesor<'_> {
    pub fn new<'a>(profesores: &'a mut Profesores) -> MenuAnadirProfesor<'a> {
        MenuAnadirProfesor { profesores }
    }

    fn _abrir_menu(&mut self, control: &Control) {
        let next_id: u32 = self._get_next_id();
        self.mostrar_texto_menu(control);
        match control.consola.pide_texto_a_usuario() {
            None => return,
            Some(nombre) => {
                let nuevo_profe = self.crear_nuevo_profe(nombre, next_id);
                self._anadir_profe(nuevo_profe, control);
            }
        }
    }

    fn mostrar_texto_menu(&self, control: &Control) {
        control.consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
    }

    fn _anadir_profe(&mut self, profesor: Profesor, control: &Control) {
        self.profesores.push(profesor);
        control.persistencia.save_profesores(&self.profesores);
    }

    fn _get_next_id(&self) -> u32 {
        let last_profe = helpers::get_last_element(&self.profesores) //
            .expect(textos::errores::NO_PROFESOR);
        last_profe.id + 1
    }

    fn crear_nuevo_profe(&self, nombre: String, id: u32) -> Profesor {
        Profesor {
            nombre,
            id,
            telefono: String::new(),
        }
    }
}

impl Menu for MenuAnadirProfesor<'_> {
    fn abrir_menu(&mut self, control: &Control) {
        self._abrir_menu(control);
    }
}

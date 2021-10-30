use crate::{
    consola,
    dominio::{teachers::Profesores, Profesor},
    helpers, repo, textos,
};

use super::Menu;

pub struct MenuAnadirProfesor<'a> {
    consola: &'a consola::Consola,
    profesores: &'a mut Profesores,
}
impl MenuAnadirProfesor<'_> {
    pub fn new<'a>(
        consola: &'a consola::Consola,
        profesores: &'a mut Profesores,
    ) -> MenuAnadirProfesor<'a> {
        MenuAnadirProfesor {
            consola: consola,
            profesores,
        }
    }
    fn mostrar_texto_menu(&self) {
        self.consola.mostrar(textos::INTRODUCE_NOMBRE_PROFESOR);
    }
    fn get_next_id(&self) -> Option<u32> {
        let next_id: u32;
        {
            let last_profe = helpers::get_last_element(&self.profesores);
            match last_profe {
                None => {
                    self.consola
                        .mostrar("Error: no se encontró ningún profesor");
                    return None;
                }
                Some(last_profe) => next_id = last_profe.id + 1,
            }
        }
        Some(next_id)
    }
    fn crear_nuevo_profe(&self, nombre: &str, id: u32) -> Profesor {
        Profesor {
            nombre: String::from(nombre),
            id,
            telefono: String::new(),
        }
    }
}

impl Menu for MenuAnadirProfesor<'_> {
    fn abrir_menu(&mut self) {
        let next_id: u32;
        {
            let posible_next_id = self.get_next_id();
            match posible_next_id {
                None => return,
                Some(id) => next_id = id,
            }
        }

        self.mostrar_texto_menu();
        let entrada_usuario = self.consola.get_input();
        let nombre_introducido = entrada_usuario.trim();
        match nombre_introducido {
            "" => return,
            _ => {
                let nuevo_profe = self.crear_nuevo_profe(nombre_introducido, next_id);
                self.profesores.push(nuevo_profe);
                repo::save_profesores(self.profesores.clone());
            }
        }
    }
}

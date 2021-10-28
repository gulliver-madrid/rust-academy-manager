use crate::asignatura::Asignaturas;
use crate::repo;
use crate::textos;
use crate::views::View;
use crate::vista;
use crate::asignatura::Asignatura;


pub struct MenuAsignaturas;

impl MenuAsignaturas {
    pub fn abrir_menu(&self, vista: &vista::Vista) {
        let mut asignaturas = repo::get_asignaturas();
        loop {
            vista.mostrar(textos::OPCIONES_MENU_ASIGNATURAS);
            let eleccion = vista.get_input();
            match eleccion.as_str() {
                "1" => self.mostrar_lista_asignaturas(&asignaturas, &vista),
                "2" => {
                    self.abrir_menu_anadir_asignatura(&mut asignaturas, &vista)
                }
                "3" => return,
                _ => continue,
            }
        }
    }

    fn mostrar_lista_asignaturas(
        &self,
        asignaturas: &Asignaturas,
        vista: &vista::Vista,
    ) {
        for asignatura in asignaturas {
            vista.mostrar(&asignatura.crear_linea_tabla());
        }
    }

    fn abrir_menu_anadir_asignatura(
        &self,
        asignaturas: &mut Asignaturas,
        vista: &vista::Vista,
    ) {
        let new_id: u32;
        {
            let last_profe = get_last_element(asignaturas);
            match last_profe {
                None => {
                    vista.mostrar("Error: no se encontró ningún profesor");
                    return;
                }
                Some(last_profe) => new_id = last_profe.id + 1,
            }
        }

        vista.mostrar(textos::INTRODUCE_NOMBRE_ASIGNATURA);
        let nombre_introducido = vista.get_input();
        match nombre_introducido.as_str() {
            "" => return,
            _ => {
                let nueva = Asignatura {
                    nombre: String::from(nombre_introducido),
                    id: new_id,
                };
                asignaturas.push(nueva);
                repo::save_asignaturas(asignaturas.clone());
            }
        }
    }
}

fn get_last_element<T>(vector: &Vec<T>) -> Option<&T> {
    let last_index = vector.len() - 1;
    let last_element = vector.get(last_index).unwrap().clone();
    Some(last_element)
}

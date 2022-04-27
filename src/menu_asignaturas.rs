use crate::asignatura::Asignaturas;
use crate::helpers;
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
            vista.clear_screen();
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
        vista.clear_screen();
        vista.mostrar("\nLista de asignaturas");
        vista.mostrar("-------------------\n");
        for asignatura in asignaturas {
            vista.mostrar(&asignatura.crear_linea_tabla());
        }
        vista.mostrar("\nPulsa ENTER para volver al menú de asignaturas");
        vista.get_input();
    }

    fn abrir_menu_anadir_asignatura(
        &self,
        asignaturas: &mut Asignaturas,
        vista: &vista::Vista,
    ) {
        let new_id: u32;
        {
            let last_profe = helpers::get_last_element(asignaturas);
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

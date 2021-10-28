mod helpers;
mod serializable;
mod serialization;
mod teachers;

use dirs::data_dir;
use helpers::set_number_chars;
use serializable::SerializableTeacher;
use teachers::{Profesor, Profesores};

use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

const DEFAULT_PROJECT_DIR: &str = "rust-academy-manager/data";
const TEACHERS_PATH: &str = "teachers.json";

fn main() {
    println!("\nPROFESORES\n");
    let menu = MenuProfesores {};
    menu.abrir_menu();

    println!("\nPrograma finalizado\n");
}

struct MenuProfesores;

impl MenuProfesores {
    fn abrir_menu(&self) {
        let serialized = read_json_profesores();
        let mut profesores = serialization::convert_serialized_to_teachers(serialized);
        loop {
            for profe in &profesores {
                println!("{}", format_profe(&profe));
            }
            loop {
                println!(
                    "\nPulsa enter para anadir un profesor, o s para salir\n"
                );
                let nombre = get_input();
                if nombre.trim() == "s" {
                    return;
                } else if nombre.trim() == "" {
                    break;
                }
            }

            self.abrir_menu_anadir_profe(&mut profesores);
        }
    }

    fn abrir_menu_anadir_profe(&self, profesores: &mut Profesores) {
        let last_index = profesores.len() - 1;
        let last_profe = profesores.get(last_index).unwrap().clone();
        let new_id = last_profe.id + 1;
        println!("Introduce el nombre de un nuevo profesor (dejalo en blanco para volver al menu principal)");
        let nombre = get_input();
        if nombre.trim() == "" {
            return;
        } else {
            let nuevo_profe = Profesor {
                nombre,
                id: new_id,
                telefono: String::new(),
            };
            profesores.push(nuevo_profe);
            let data_to_serialize =
                serialization::convert_teachers_to_serializable(
                    profesores.clone(),
                );
            let serialized =
                serde_json::to_string_pretty(&data_to_serialize).unwrap();
            write_in_file(&get_teachers_path(), serialized);
        }
    }
}

fn get_teachers_path() -> PathBuf {
    let mut path = get_project_data_path();
    path.push(TEACHERS_PATH);
    path
}

fn get_project_data_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(data_dir().unwrap());
    path.push(DEFAULT_PROJECT_DIR);
    path
}

fn write_in_file(ruta: &PathBuf, texto: String) {
    let mut file = OpenOptions::new() //
        .write(true)
        .truncate(true)
        .open(ruta)
        .unwrap();
    writeln!(&mut file, "{}", texto.as_str()).unwrap();
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    let input = String::from(input.trim());
    input
}

fn read_json_profesores() -> Vec<SerializableTeacher> {
    let file = File::open(get_teachers_path()).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn format_profe(profe: &Profesor) -> String {
    let profe_str = set_number_chars(&profe.nombre, 22);
    let tlf_str = match profe.telefono.as_str() {
        "" => "desconocido",
        otro => otro,
    };
    format!(
        "Nombre: {}  Id: {}  Telefono: {}",
        profe_str,
        format!("{:0>3}", profe.id),
        tlf_str
    )
}

mod helpers;
mod serializable;
mod serialization;
mod teachers;

use dirs::data_dir;
use helpers::set_number_chars;
use serializable::SerializableTeacher;
use teachers::Profesor;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

const DEFAULT_PROJECT_DIR: &str = "rust-academy-manager/data";
const TEACHERS_PATH: &str = "teachers.json";

use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    println!("\nPROFESORES\n");
    let serialized = read_json_profesores();
    let mut datos = serialization::convert_serialized_to_teachers(serialized);

    loop {
        for profe in &datos {
            println!("{}", format_profe(profe.clone()));
        }
        let last_index = datos.len() - 1;
        let last_profe = &datos.get(last_index).unwrap().clone();
        let new_id = last_profe.id + 1;
        println!("Introduce el nombre de un nuevo profesor (dejalo en blanco para salir)");
        let nombre = get_input();
        if nombre.trim() == "" {
            break;
        } else {
            let nuevo_profe = Profesor {
                nombre,
                id: new_id,
                telefono: String::new(),
            };
            datos.push(nuevo_profe);
            let data_to_serialize =
                serialization::convert_teachers_to_serializable(datos.clone());
            let serialized =
                serde_json::to_string_pretty(&data_to_serialize).unwrap();
            write_in_file(&get_teachers_path(), serialized);
        }
    }

    println!("\nPrograma finalizado\n");
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

fn format_profe(profe: Profesor) -> String {
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

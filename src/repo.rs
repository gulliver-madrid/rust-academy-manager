use std::fs::File;

use dirs::data_dir;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

use crate::serializable::SerializableTeacher;
use crate::serialization;
use crate::teachers::Profesores;

const DEFAULT_PROJECT_DIR: &str = "rust-academy-manager/data";
const TEACHERS_PATH: &str = "teachers.json";

pub fn save_profesores(profesores: Profesores) {
    let data_to_serialize =
        serialization::convert_teachers_to_serializable(profesores.clone());
    let json = serde_json::to_string_pretty(&data_to_serialize).unwrap();
    self::write_in_file(&get_teachers_path(), json);
}

pub fn get_profesores() -> Profesores {
    let serialized = read_json_profesores();
    let profesores = serialization::convert_serialized_to_teachers(serialized);
    profesores
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

fn read_json_profesores() -> Vec<SerializableTeacher> {
    let file = File::open(get_teachers_path()).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

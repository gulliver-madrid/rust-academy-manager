use std::fs::File;

use dirs::data_dir;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

use crate::asignatura::Asignaturas;
use crate::serializable::{SerializableSubject, SerializableTeacher};
use crate::serialization;
use crate::teachers::Profesores;

const DEFAULT_PROJECT_DIR: &str = "rust-academy-manager/data";
const TEACHERS_PATH: &str = "teachers.json";
const SUBJECTS_PATH: &str = "subjects.json";

pub fn save_profesores(profesores: Profesores) {
    let data_to_serialize =
        serialization::convert_teachers_to_serializable(profesores.clone());
    let json = serde_json::to_string_pretty(&data_to_serialize).unwrap();
    self::write_in_file(&get_teachers_path(), json);
}
pub fn save_asignaturas(asignaturas: Asignaturas) {
    let data_to_serialize =
        serialization::convert_subjects_to_serializable(asignaturas.clone());
    let json = serde_json::to_string_pretty(&data_to_serialize).unwrap();
    self::write_in_file(&get_subjects_path(), json);
}

pub fn get_profesores() -> Profesores {
    let serialized = read_json_profesores();
    let profesores = serialization::convert_serialized_to_teachers(serialized);
    profesores
}
pub fn get_asignaturas() -> Asignaturas {
    let serialized = read_json_asignaturas();
    let asignaturas = serialization::convert_serialized_to_subjects(serialized);
    asignaturas
}

fn get_teachers_path() -> PathBuf {
    let mut path = get_project_data_path();
    path.push(TEACHERS_PATH);
    path
}
fn get_subjects_path() -> PathBuf {
    let mut path = get_project_data_path();
    path.push(SUBJECTS_PATH);
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

pub fn read_json_asignaturas() -> Vec<SerializableSubject> {
    let file = File::open(get_subjects_path()).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

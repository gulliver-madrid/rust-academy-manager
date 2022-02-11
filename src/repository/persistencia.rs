use std::fs::File;

use dirs::data_dir;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

use super::serialization;
use crate::dominio::Asignaturas;
use crate::dominio::Profesores;
use super::serializable::{SerializableSubject, SerializableTeacher};

const DEFAULT_PROJECT_DIR: &str = "rust-academy-manager/data";
const TEACHERS_PATH: &str = "teachers.json";
const SUBJECTS_PATH: &str = "subjects.json";

pub trait PersistenciaTrait {
    fn save_profesores(&self, profesores: &Profesores);
    fn save_asignaturas(&self, asignaturas: &Asignaturas);
    fn load_profesores(&self) -> Profesores;
    fn load_subjects(&self) -> Asignaturas;
}
pub struct Persistencia {}

impl PersistenciaTrait for Persistencia {
    fn save_profesores(&self, profesores: &Profesores) {
        let data_to_serialize = serialization::convert_teachers_to_serializable(profesores.clone());
        let json = to_json(&data_to_serialize);
        self::write_in_file(&get_teachers_path(), json);
    }
    fn save_asignaturas(&self, asignaturas: &Asignaturas) {
        let data_to_serialize =
            serialization::convert_subjects_to_serializable(asignaturas.clone());
        let json = to_json(&data_to_serialize);
        self::write_in_file(&get_subjects_path(), json);
    }

    fn load_profesores(&self) -> Profesores {
        let serialized = read_json_profesores();
        let profesores = serialization::convert_serialized_to_teachers(serialized);
        profesores
    }

    fn load_subjects(&self) -> Asignaturas {
        let serialized = read_json_asignaturas();
        let asignaturas = serialization::convert_serialized_to_subjects(serialized);
        asignaturas
    }
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

fn to_json<T>(item: &T) -> String
where
    T: ?Sized + Serialize,
{
    serde_json::to_string_pretty(item).unwrap()
}

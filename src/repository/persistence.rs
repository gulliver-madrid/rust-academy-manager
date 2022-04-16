use std::fs::File;

use dirs::data_dir;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

use super::serializable::{SerializableSubject, SerializableTeacher};
use super::serialization;
use crate::domain::Subjects;
use crate::domain::Teachers;

use super::PersistenceTrait;

const DEFAULT_PROJECT_DIR: &str = "rust-academy-manager/data";
const TEACHERS_PATH: &str = "teachers.json";
const SUBJECTS_PATH: &str = "subjects.json";



fn get_project_data_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(data_dir().unwrap());
    path.push(DEFAULT_PROJECT_DIR);
    path
}
pub struct JsonPersistence {}

impl JsonPersistence {
    fn get_teachers_path(&self) -> PathBuf {
        let mut path = get_project_data_path();
        path.push(TEACHERS_PATH);
        path
    }
    fn get_subjects_path(&self) -> PathBuf {
        let mut path = get_project_data_path();
        path.push(SUBJECTS_PATH);
        path
    }

    fn read_json_teachers(&self) -> Vec<SerializableTeacher> {
        let file = File::open(self.get_teachers_path()).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    pub fn read_json_subjects(&self) -> Vec<SerializableSubject> {
        let file = File::open(self.get_subjects_path()).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }
}

impl PersistenceTrait for JsonPersistence {
    fn save_teachers(&self, teachers: &Teachers) {
        let data_to_serialize =
            serialization::convert_teachers_to_serializable(teachers.clone());
        let json = to_json(&data_to_serialize);
        write_in_file(&self.get_teachers_path(), json);
    }
    fn save_subjects(&self, subjects: &Subjects) {
        let data_to_serialize =
            serialization::convert_subjects_to_serializable(subjects.clone());
        let json = to_json(&data_to_serialize);
        write_in_file(&self.get_subjects_path(), json);
    }

    fn load_teachers(&self) -> Teachers {
        let serialized = self.read_json_teachers();
        let teachers = serialization::convert_serialized_to_teachers(serialized);
        teachers
    }

    fn load_subjects(&self) -> Subjects {
        let serialized = self.read_json_subjects();
        let subjects = serialization::convert_serialized_to_subjects(serialized);
        subjects
    }
}

fn write_in_file(ruta: &PathBuf, texto: String) {
    let mut file = OpenOptions::new() //
        .write(true)
        .truncate(true)
        .open(ruta)
        .unwrap();
    writeln!(&mut file, "{}", texto.as_str()).unwrap();
}




fn to_json<T>(item: &T) -> String
where
    T: ?Sized + Serialize,
{
    serde_json::to_string_pretty(item).unwrap()
}

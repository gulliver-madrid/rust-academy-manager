use crate::asignatura::Asignatura;
use crate::serializable::SerializableSubject;

use super::teachers::Profesor;
use super::serializable::SerializableTeacher;

pub fn convert_teachers_to_serializable(
    profesores: Vec<Profesor>,
) -> Vec<SerializableTeacher> {
    let mut teachers = Vec::new();
    for profesor in profesores {
        teachers.push(SerializableTeacher {
            name: profesor.nombre,
            id: profesor.id,
            phone_number: profesor.telefono,
        })
    }
    teachers
}
pub fn convert_serialized_to_teachers(
    teachers: Vec<SerializableTeacher>,
) -> Vec<Profesor> {
    let mut profesores = Vec::new();
    for teacher in teachers {
        profesores.push(Profesor {
            nombre: teacher.name,
            id: teacher.id,
            telefono: teacher.phone_number,
        })
    }
    profesores
}


pub fn convert_subjects_to_serializable(
    asignaturas: Vec<Asignatura>,
) -> Vec<SerializableSubject> {
    let mut subjects = Vec::new();
    for asignatura in asignaturas {
        subjects.push(SerializableSubject {
            name: asignatura.nombre,
            id: asignatura.id,
        })
    }
    subjects
}
pub fn convert_serialized_to_subjects(
    subjects: Vec<SerializableSubject>,
) -> Vec<Asignatura> {
    let mut asignaturas = Vec::new();
    for subject in subjects {
        asignaturas.push(Asignatura {
            nombre: subject.name,
            id: subject.id,
        })
    }
    asignaturas
}
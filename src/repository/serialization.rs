use super::serializable::{SerializableSubject, SerializableTeacher};
use crate::dominio::{asignatura::Asignatura, teachers::Profesor};

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
            assigned_teachers: asignatura.profesores_asignados,
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
            profesores_asignados: subject.assigned_teachers,
        })
    }
    asignaturas
}

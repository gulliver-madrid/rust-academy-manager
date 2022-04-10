use super::serializable::{SerializableSubject, SerializableTeacher};
use crate::domain::{Subject, Teacher};

pub fn convert_teachers_to_serializable(
    teachers: Vec<Teacher>,
) -> Vec<SerializableTeacher> {
    let mut serializable_teachers = Vec::new();
    for teacher in teachers {
        serializable_teachers.push(SerializableTeacher {
            name: teacher.name,
            id: teacher.id,
            phone_number: teacher.phone_number,
        })
    }
    serializable_teachers
}
pub fn convert_serialized_to_teachers(
    serializable_teachers: Vec<SerializableTeacher>,
) -> Vec<Teacher> {
    let mut teachers = Vec::new();
    for serializable in serializable_teachers {
        teachers.push(Teacher {
            name: serializable.name,
            id: serializable.id,
            phone_number: serializable.phone_number,
        })
    }
    teachers
}
pub fn convert_subjects_to_serializable(
    subjects: Vec<Subject>,
) -> Vec<SerializableSubject> {
    let mut serializable_subjects = Vec::new();
    for subject in subjects {
        serializable_subjects.push(SerializableSubject {
            name: subject.name,
            id: subject.id,
            assigned_teachers: subject.assigned_teachers,
        })
    }
    serializable_subjects
}
pub fn convert_serialized_to_subjects(
    serializable_subjects: Vec<SerializableSubject>,
) -> Vec<Subject> {
    let mut subjects = Vec::new();
    for serializable in serializable_subjects {
        subjects.push(Subject {
            name: serializable.name,
            id: serializable.id,
            assigned_teachers: serializable.assigned_teachers,
        })
    }
    subjects
}

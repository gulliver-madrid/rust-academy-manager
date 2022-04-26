use super::Profesor;
use super::SerializableTeacher;
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

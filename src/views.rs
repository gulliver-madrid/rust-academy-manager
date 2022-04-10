use crate::domain::Subject;
use crate::domain::Teacher;
use crate::helpers::set_number_chars;

const NAME_TAG: &str = "Nombre";
const ID_TAG: &str = "Id";
const PHONE_TAG: &str = "Telefono";
const SUBJECT_TAG: &str = "Asignatura";

pub trait View {
    fn create_table_row(&self) -> String;
}
impl View for Teacher {
    fn create_table_row(&self) -> String {
        let teacher_str = set_number_chars(&self.name, 22);
        let phone_str = match self.phone_number.as_str() {
            "" => "desconocido",
            otro => otro,
        };
        format!(
            "{}: {}  {}: {}  {}: {}",
            NAME_TAG,
            teacher_str,
            ID_TAG,
            format!("{:0>3}", self.id),
            PHONE_TAG,
            phone_str
        )
    }
}
impl View for Subject {
    fn create_table_row(&self) -> String {
        let asignatura_str = set_number_chars(&self.name, 28);

        format!(
            "{}: {}  {}: {}",
            SUBJECT_TAG,
            asignatura_str,
            ID_TAG,
            format!("{:0>3}", self.id)
        )
    }
}

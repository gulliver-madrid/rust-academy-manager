use crate::domain::Subject;
use crate::domain::Teacher;
use crate::helpers::set_number_chars;
use rust_i18n::t;



pub trait View {
    fn create_table_row(&self) -> String;
}
impl View for Teacher {
    fn create_table_row(&self) -> String {
        let teacher_str = set_number_chars(&self.name, 22);
        let unknown_phone_text = &t!("unknown");
        let phone_str = match self.phone_number.as_str() {
            "" => unknown_phone_text,
            otro => otro,
        };
        format!(
            "{}: {}  {}: {}  {}: {}",
            t!("tags.name"),
            teacher_str,
            t!("tags.id"),
            format!("{:0>3}", self.id),
            t!("tags.phone"),
            phone_str
        )
    }
}
impl View for Subject {
    fn create_table_row(&self) -> String {
        let asignatura_str = set_number_chars(&self.name, 28);

        format!(
            "{}: {}  {}: {}",
            t!("tags.subject"),
            asignatura_str,
            t!("tags.id"),
            format!("{:0>3}", self.id)
        )
    }
}

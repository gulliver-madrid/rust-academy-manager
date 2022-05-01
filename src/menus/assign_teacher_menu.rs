use rust_i18n::t;

use crate::{
    components::Control,
    errors::{SimpleError, SimpleResult},
};

/// Menu for assign a teacher to a subject
pub struct AssignTeacherMenu<'a> {
    pub control: &'a mut Control,
}

impl AssignTeacherMenu<'_> {
    pub fn open_menu(&mut self) {
        let ui = &self.control.ui;
        ui.show(t!("assign_teacher_menu.choose_subject"));
        let subject_name: String;
        if let Some(entered_text) = ui.ask_text_to_user() {
            subject_name = entered_text
        } else {
            ui.show(t!("cancelled_op"));
            ui.pause_enter(&t!("continue"));
            return;
        }

        let subject_index: usize;
        match self.get_subject_index(&subject_name) {
            Ok(index) => subject_index = index,
            Err(e) => {
                ui.show(e.to_string());
                ui.pause_enter(&t!("continue"));
                return;
            }
        }

        ui.show(introduced_subject_msg(&subject_name));
        ui.show(t!("assign_teacher_menu.ask_teacher_id"));
        if let Some(entered_text) = ui.ask_text_to_user() {
            let teacher_id = entered_text.parse::<u32>().unwrap();
            let result = self
                .control
                .application
                .assign_teacher_to_subject(subject_index, teacher_id);
            ui.show(result_msg(result));
            ui.pause_enter(&t!("continue"));
        }
    }

    fn get_subject_index(&self, subject_name: &str) -> Result<usize, SimpleError> {
        self.control
            .application
            .subjects_app
            .get_subject_index_by_name(subject_name)
    }
}

fn introduced_subject_msg(subject_name: &str) -> String {
    format!(
        "{}: {}",
        &t!("assign_teacher_menu.introduced_subject"),
        subject_name
    )
}

fn result_msg(result: SimpleResult) -> String {
    if result.is_ok() {
        t!("assign_teacher_menu.ok")
    } else {
        t!("assign_teacher_menu.coudnt_op")
    }
}

use std::rc::Rc;

use rust_i18n::t;

use crate::{
    components::Control,
    errors::{SimpleError, SimpleResult},
};

/// Menu for assign a teacher to a subject
pub struct AssignTeacherMenu {
    pub control: Rc<Control>,
}

impl AssignTeacherMenu {
    pub fn open_menu(&mut self) {
        self.control
            .application
            .teachers_app
            .borrow()
            .load_teachers_if_needed();
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
        loop {
            ui.show(t!("assign_teacher_menu.ask_teacher_id"));
            if let Some(entered_text) = ui.ask_text_to_user() {
                if let Some(teacher_id) = entered_text.parse::<u32>().ok() {
                    let control = Rc::clone(&self.control);
                    let result = control
                        .application
                        .assign_teacher_to_subject(subject_index, teacher_id);
                    ui.show(result_msg(result));
                    ui.pause_enter(&t!("continue"));
                    break;
                }
            }
            ui.show(t!("teacher_id_should_be_a_number"));
            ui.show(t!("ask_if_cancel_op"));
            match ui.ask_text_to_user() {
                Some(answer) => {
                    if answer == t!("option_yes_one_char") {
                        break;
                    }
                }
                None => (),
            }
        }
    }

    fn get_subject_index(&self, subject_name: &str) -> Result<usize, SimpleError> {
        self.control
            .application
            .subjects_app
            .borrow()
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
    match result {
        Ok(_) => t!("assign_teacher_menu.ok"),
        Err(e) => format!("{}: {}", t!("assign_teacher_menu.coudnt_op"), e.to_string()),
    }
}

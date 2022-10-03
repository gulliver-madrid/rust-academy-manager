use std::rc::Rc;

use rust_i18n::t;

use crate::{
    components::Control,
    errors::{SimpleError, SimpleResult},
    helpers::{contains_some_digits, only_contains_digits},
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
            let user_input = ui.ask_text_to_user();
            let result = validate_teacher_id(&user_input);
            match result {
                Ok(teacher_id) => {
                    let result =
                        self.assign_teacher_to_subject(subject_index, teacher_id);
                    ui.show(result_msg(result));
                    ui.pause_enter(&t!("continue"));
                    break;
                }
                Err(msg) => ui.show(msg),
            }

            ui.show(t!("ask_if_cancel_op"));
            if let Some(answer) = ui.ask_text_to_user() {
                if answer == t!("option_yes_one_char") {
                    break;
                }
            }
        }
    }

    fn assign_teacher_to_subject(
        &self,
        subject_index: usize,
        teacher_id: u32,
    ) -> SimpleResult {
        self.control
            .application
            .assign_teacher_to_subject(subject_index, teacher_id)
    }

    fn get_subject_index(&self, subject_name: &str) -> Result<usize, SimpleError> {
        self.control
            .application
            .subjects_app
            .borrow()
            .get_subject_index_by_name(subject_name)
    }
}

type ParseIdResult = Result<u32, String>;

pub fn validate_teacher_id(user_input: &Option<String>) -> ParseIdResult {
    let trimmed_input = user_input.as_ref().map(|s| s.trim().to_string());
    match trimmed_input {
        Some(entered_text) => {
            if entered_text.is_empty() {
                return Err(t!("no_id_was_entered"));
            }
            if only_contains_digits(&entered_text) {
                return parse_numeric_id(&entered_text);
            }
            if contains_some_digits(&entered_text) {
                return Err(t!("id_should_have_only_digits"));
            }
            Err(t!("teacher_id_should_be_a_number"))
        }
        None => Err(t!("no_id_was_entered")),
    }
}
fn parse_numeric_id(entered_text: &str) -> ParseIdResult {
    match entered_text.parse::<u32>().ok() {
        Some(teacher_id) => Ok(teacher_id),
        None => Err(t!("no_valid_id")),
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

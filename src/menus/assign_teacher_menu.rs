use rust_i18n::t;

use crate::components::Control;

/// Menu for assign a teacher to a subject
pub struct AssignTeacherMenu<'a> {
    pub control: &'a mut Control,
}

impl AssignTeacherMenu<'_> {
    pub fn open_menu(&mut self) {
        let ui = &self.control.ui;
        ui.show(&t!("assign_teacher_menu.choose_subject"));
        let subject_name: String;
        match ui.ask_text_to_user() {
            Some(entered_text) => subject_name = entered_text,
            None => {
                ui.show(&t!("cancelled_op"));
                ui.pause_enter(&t!("continue"));
                return;
            }
        }
        let subject_index: usize;
        match self
            .control
            .application
            .subjects_app
            .get_subject_index_by_name(&subject_name)
        {
            Ok(index) => {
                subject_index = index;
            }
            Err(e) => {
                ui.show(&e.to_string());
                ui.pause_enter(&t!("continue"));
                return;
            }
        }
        // Show introduced subject
        ui.show(&format!(
            "{}: {}",
            &t!("assign_teacher_menu.introduced_subject"),
            subject_name
        ));
        ui.show(&t!("assign_teacher_menu.ask_teacher_id"));
        if let Some(entered_text) = ui.ask_text_to_user() {
            let teacher_id = entered_text.parse::<u32>().unwrap();
            let result = self
                .control
                .application
                .assign_teacher_to_subject(subject_index, teacher_id);
            if result.is_ok() {
                ui.show(&t!("assign_teacher_menu.ok"));
            } else {
                ui.show(&t!("assign_teacher_menu.coudnt_op"));
            }
            ui.pause_enter(&t!("continue"));
        }
    }
}

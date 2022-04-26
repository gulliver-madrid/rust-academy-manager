use rust_i18n::t;

use crate::{components::Control, errors::SimpleResult};

pub struct RemoveSubjectMenu<'a> {
    pub control: &'a mut Control,
}

impl RemoveSubjectMenu<'_> {
    pub fn open_menu(&mut self) {
        let ui = &self.control.ui;
        self.show_menu_text();
        match ui.ask_text_to_user() {
            None => (),
            Some(name) => {
                let result = self.control.application.subjects_app.remove_subject(&name);
                let msg = self.get_info_result(result, &name);
                ui.show(&msg);
                ui.pause_enter(&t!("continue"));
            }
        }
    }

    fn show_menu_text(&self) {
        self.control
            .ui
            .show(&t!("enter_name_subject_to_be_deleted"));
    }

    fn get_info_result(&self, result: SimpleResult, name: &str) -> String {
        match result {
            Ok(_) => format!(
                //
                "{} {}",
                t!("successfully_removed_subject"),
                name
            ),
            Err(e) => e.to_string(),
        }
    }
}

use crate::{components::Control, errors::SimpleResult, texts};

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
                let result = self.control.application.remove_subject(&name);
                let msg = self.get_info_result(result, &name);
                ui.show(&msg);
                ui.pause_enter(texts::CONTINUE);
            }
        }
    }

    fn show_menu_text(&self) {
        self.control
            .ui
            .show(texts::ENTER_NAME_SUBJECT_TO_BE_DELETED);
    }

    fn get_info_result(&self, result: SimpleResult, name: &str) -> String {
        match result {
            Ok(_) => format!("Se eliminó exitosamente la asignatura {}", name),
            Err(e) => e.to_string(),
        }
    }
}

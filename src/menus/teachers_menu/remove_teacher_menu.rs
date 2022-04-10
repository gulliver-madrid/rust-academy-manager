use crate::{components::Control, errors::SimpleResult, texts};

pub struct RemoveTeacherMenu<'a> {
    pub control: &'a mut Control,
}

impl RemoveTeacherMenu<'_> {
    pub fn open_menu(&mut self) {
        let ui = &self.control.ui;
        self.show_menu_text();

        if let Some(name) = ui.ask_text_to_user() {
            let result = self
                .control
                .application
                .teachers_app
                .remove_teacher(&name);
            let msg = self.get_info_result(result, name);
            ui.show(&msg);
            ui.pause_enter(texts::CONTINUE);
        };
    }

    fn show_menu_text(&self) {
        self.control
            .ui
            .show(texts::ENTER_NAME_TEACHER_TO_BE_DELETED);
    }

    fn get_info_result(&self, result: SimpleResult, name: String) -> String {
        match result {
            Ok(_) => format!("Se eliminó exitosamente al profesor {}", name),
            Err(e) => e.to_string(),
        }
    }
}

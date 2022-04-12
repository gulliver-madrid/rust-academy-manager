use rust_i18n::t;

use crate::{components::Control};

pub struct AddTeacherMenu<'a> {
    pub control: &'a mut Control,
}

impl AddTeacherMenu<'_> {
    pub fn open_menu(&mut self) {
        let ui = &self.control.ui;
        self.show_menu_text();
        if let Some(name) = ui.ask_text_to_user() {
            let result = self
                .control
                .application
                .teachers_app
                .add_new_teacher(&name);
            let msg = match result {
                Ok(_) => format!("Profesor con nombre {} añadido", name),
                Err(e) => e.to_string(),
            };
            ui.show(&msg);
            ui.pause_enter(&t!("continue"));
        }
    }

    fn show_menu_text(&self) {
        self.control.ui.show(&t!("input_teacher_name"));
    }
}

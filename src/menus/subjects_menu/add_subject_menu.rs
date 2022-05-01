use rust_i18n::t;

use crate::components::Control;

pub struct AddSubjectMenu<'a> {
    pub control: &'a mut Control,
}

impl AddSubjectMenu<'_> {
    pub fn open_menu(&mut self) {
        let ui = &self.control.ui;
        self.show_menu_text();
        if let Some(name) = ui.ask_text_to_user() {
            let result = self.control.application.subjects_app.add_new_subject(&name);
            let msg = match result {
                Ok(_) => format!(
                    "{} {} {}",
                    t!("add_subject_menu.subject"),
                    name,
                    t!("add_subject_menu.added")
                ),
                Err(e) => e.to_string(),
            };
            ui.show(&msg);
            ui.pause_enter(&t!("continue"));
        }
    }

    fn show_menu_text(&self) {
        self.control.ui.show(&t!("input_subject_name"));
    }
}

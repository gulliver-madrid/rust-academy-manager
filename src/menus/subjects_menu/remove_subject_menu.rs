use std::rc::Rc;

use rust_i18n::t;

use crate::{components::Control, errors::SimpleResult};

pub struct RemoveSubjectMenu {
    pub control: Rc<Control>,
}

impl RemoveSubjectMenu {
    pub fn open_menu(&self) {
        let ui = &self.control.ui;
        self.show_menu_text();
        if let Some(name) = ui.ask_text_to_user() {
            let result = self
                .control
                .application
                .subjects_app
                .borrow()
                .remove_subject(&name);
            let msg = get_info_result(result, &name);
            ui.show(msg);
            ui.pause_enter(&t!("continue"));
        }
    }

    fn show_menu_text(&self) {
        self.control.ui.show(t!("enter_name_subject_to_be_deleted"));
    }
}

fn get_info_result(result: SimpleResult, name: &str) -> String {
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

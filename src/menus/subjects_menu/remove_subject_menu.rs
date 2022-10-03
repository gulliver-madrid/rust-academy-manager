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

        let name = match ui.ask_text_to_user() {
            Some(entered_text) => entered_text,
            None => return,
        };
        let result = self.remove_subject(&name);
        let msg = get_info_result(result, &name);
        ui.show(msg);
        ui.pause_enter(&t!("continue"));
    }
    fn show_menu_text(&self) {
        self.control.ui.show(t!("enter_name_subject_to_be_deleted"));
    }
    fn remove_subject(&self, name: &str) -> SimpleResult {
        let result = self
            .control
            .application
            .subjects_app
            .borrow()
            .remove_subject(name);
        result
    }
}

fn get_info_result(result: SimpleResult, name: &str) -> String {
    match result {
        Ok(_) => get_success_msg(name),
        Err(e) => e.to_string(),
    }
}
fn get_success_msg(name: &str) -> String {
    format!(
        "{} {}", // fmt
        t!("successfully_removed_subject"),
        name
    )
}

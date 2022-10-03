use std::rc::Rc;

use rust_i18n::t;

use crate::{components::Control, errors::SimpleResult};

pub struct AddSubjectMenu {
    pub control: Rc<Control>,
}

impl AddSubjectMenu {
    pub fn open_menu(&self) {
        let ui = &self.control.ui;
        self.show_menu_text();
        let name = match ui.ask_text_to_user() {
            Some(entered_text) => entered_text,
            None => return,
        };

        let result = self.add_new_subject(&name);
        let msg = get_info_result(result, &name);
        ui.show(msg);
        ui.pause_enter(&t!("continue"));
    }

    fn show_menu_text(&self) {
        self.control.ui.show(t!("input_subject_name"));
    }

    fn add_new_subject(&self, name: &str) -> SimpleResult {
        let result = self
            .control
            .application
            .subjects_app
            .borrow()
            .add_new_subject(name);
        result
    }
}

fn get_info_result(result: SimpleResult, name: &str) -> String {
    let msg = match result {
        Ok(_) => get_success_msg(name),
        Err(e) => e.to_string(),
    };
    msg
}
fn get_success_msg(name: &str) -> String {
    format!(
        "{} {} {}",
        t!("add_subject_menu.subject"),
        name,
        t!("add_subject_menu.added")
    )
}

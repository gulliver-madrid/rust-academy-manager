use std::rc::Rc;

use rust_i18n::t;

use crate::{components::Control, errors::SimpleResult};

pub struct RemoveTeacherMenu {
    control: Rc<Control>,
}

impl RemoveTeacherMenu {
    pub fn new(control: &Rc<Control>) -> Self {
        Self {
            control: Rc::clone(control),
        }
    }
    pub fn open_menu(&self) {
        let ui = &self.control.ui;
        ui.show(t!("enter_name_teacher_to_be_deleted"));
        if let Some(name) = ui.ask_text_to_user() {
            let result = self.remove_teacher(&name);
            let msg = self.get_info_result(result, name);
            ui.show(msg);
            ui.pause_enter(&t!("continue"));
        };
    }

    fn remove_teacher(&self, name: &str) -> SimpleResult {
        self.control
            .application
            .teachers_app
            .borrow()
            .remove_teacher(name)
    }

    fn get_info_result(&self, result: SimpleResult, name: String) -> String {
        match result {
            Ok(_) => get_success_msg(&name),
            Err(e) => e.to_string(),
        }
    }
}

fn get_success_msg(name: &str) -> String {
    format!(
        "{} {} {}",
        t!("successfully_removed_teacher.before"),
        name,
        t!("successfully_removed_teacher.after")
    )
}

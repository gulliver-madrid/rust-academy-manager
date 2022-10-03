use std::rc::Rc;

use rust_i18n::t;

use crate::{components::Control, errors::SimpleResult};

pub struct RemoveTeacherMenu {
    pub control: Rc<Control>,
}

impl RemoveTeacherMenu {
    pub fn open_menu(&self) {
        let ui = &self.control.ui;
        ui.show(menu_text());
        let name = match ui.ask_text_to_user() {
            Some(entered_text) => entered_text,
            None => return,
        };

        let result = self
            .control
            .application
            .teachers_app
            .borrow()
            .remove_teacher(&name);
        let msg = self.get_info_result(result, name);
        ui.show(msg);
        ui.pause_enter(&t!("continue"));
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

fn menu_text() -> String {
    t!("enter_name_teacher_to_be_deleted")
}

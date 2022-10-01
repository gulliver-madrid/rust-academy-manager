use std::rc::Rc;

use rust_i18n::t;

use crate::components::Control;

pub struct AddTeacherMenu {
    pub control: Rc<Control>,
}

impl AddTeacherMenu {
    pub fn open_menu(&self) {
        let ui = &self.control.ui;
        self.show_menu_text();
        if let Some(name) = ui.ask_text_to_user() {
            let result = self
                .control
                .application
                .teachers_app
                .borrow()
                .add_new_teacher(&name);
            let msg = match result {
                Ok(_) => get_msg_success(&name),
                Err(e) => e.to_string(),
            };
            ui.show(msg);
            ui.pause_enter(&t!("continue"));
        }
    }

    fn show_menu_text(&self) {
        self.control.ui.show(t!("input_teacher_name"));
    }
}

fn get_msg_success(name: &str) -> String {
    format!(
        "{} {} {}",
        t!("add_teacher_success.before"),
        name,
        t!("add_teacher_success.after")
    )
}

use std::rc::Rc;

use rust_i18n::t;

use crate::{components::Control, errors::SimpleResult};

pub struct AddTeacherMenu {
    control: Rc<Control>,
}

impl AddTeacherMenu {
    pub fn new(control: &Rc<Control>) -> Self {
        Self {
            control: Rc::clone(control),
        }
    }
    pub fn open_menu(&self) {
        let ui = &self.control.ui;
        self.show_menu_text();
        if let Some(name) = ui.ask_text_to_user() {
            let result = self.add_new_teacher(&name);
            let msg = match result {
                Ok(_) => get_success_msg(&name),
                Err(e) => e.to_string(),
            };
            ui.show(msg);
            ui.pause_enter(&t!("continue"));
        }
    }

    fn add_new_teacher(&self, name: &str) -> SimpleResult {
        self.control
            .application
            .teachers_app
            .borrow()
            .add_new_teacher(name)
    }

    fn show_menu_text(&self) {
        self.control.ui.show(t!("input_teacher_name"));
    }
}

fn get_success_msg(name: &str) -> String {
    format!(
        "{} {} {}",
        t!("add_teacher_success.before"),
        name,
        t!("add_teacher_success.after")
    )
}

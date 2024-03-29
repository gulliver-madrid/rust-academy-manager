use rust_i18n::t;

use super::application::Application;
use crate::ui::UserInterface;

pub struct Control {
    pub ui: UserInterface,
    pub application: Application,
}

#[allow(unused)]
pub fn warn_not_implemented(control: Control) {
    control.ui.show(t!("errors.not_implemented"))
}

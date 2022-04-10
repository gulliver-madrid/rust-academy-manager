use crate::{application::Application, ui::UserInterface};

pub struct Control {
    pub ui: UserInterface,
    pub application: Application,
}

#[allow(unused)]
pub fn warn_not_implemented(control: &mut Control) {
    control.ui.show("ERROR: No implementado")
}

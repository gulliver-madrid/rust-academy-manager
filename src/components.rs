use crate::{ui::{UserInterface}, application::Application};

pub struct Control {
    pub ui: UserInterface,
    pub application: Application,
}



#[allow(unused)]
pub fn warn_not_implemented(control: &mut Control) {
    control.ui.mostrar("ERROR: No implementado")
}

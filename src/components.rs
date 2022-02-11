use crate::{consola::{Consola}, application::Application};

pub struct Control {
    pub consola: Consola,
    pub application: Application,
}



#[allow(unused)]
pub fn warn_not_implemented(control: &mut Control) {
    control.consola.mostrar("ERROR: No implementado")
}

use crate::{consola::Consola, repository::Repository};

pub struct Control {
    pub consola: Consola,
    pub repository: Repository,
}

pub trait Component {
    fn render(&mut self, control: &Control);
}

#[allow(unused)]
pub fn warn_not_implemented(control: &Control) {
    control.consola.mostrar("ERROR: No implementado")
}

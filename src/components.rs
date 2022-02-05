use crate::{consola::Consola, persistencia::Persistencia};

pub struct Control {
    pub consola: Consola,
    pub persistencia: Persistencia,
}

pub trait Component {
    fn render(&mut self, control: &Control);
}

#[allow(unused)]
pub fn warn_not_implemented(control: &Control) {
    control.consola.mostrar("ERROR: No implementado")
}

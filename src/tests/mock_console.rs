#[cfg(test)]
use std::{cell::RefCell};

#[cfg(test)] // El compilador solo lo detecta en modo test
use crate::{
    ui::{InnerConsole},
};


#[cfg(test)]
pub const SHOW_CONSOLE_OUTPUT: bool = false;
#[cfg(test)]
pub struct MockConsole {
    pub provided_inputs: RefCell<Vec<String>>,
}

#[cfg(test)]
impl MockConsole {
    pub fn new() -> Self {
        Self {
            provided_inputs: RefCell::new(Vec::<String>::new()),
        }
    }
}
#[cfg(test)]
impl InnerConsole for MockConsole {
    fn clear_screen(&self) {}
    fn get_input(&self) -> String {
        self.provided_inputs.borrow_mut().pop().unwrap()
    }
    fn mostrar(&self, texto: &str) {
        if SHOW_CONSOLE_OUTPUT {
            println!("{}", texto);
        }
    }
}
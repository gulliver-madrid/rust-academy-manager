#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
use std::collections::VecDeque;

#[cfg(test)]
use crate::ui::InnerConsole;

#[cfg(test)]
const SHOW_CONSOLE_OUTPUT: bool = false;

#[cfg(test)]
pub struct MockConsole {
    provided_inputs: RefCell<VecDeque<String>>,
    outputs: RefCell<Vec<String>>,
}

#[cfg(test)]
impl MockConsole {
    pub fn new() -> Self {
        Self {
            provided_inputs: RefCell::new(VecDeque::<String>::new()),
            outputs: RefCell::new(Vec::<String>::new()),
        }
    }
    pub fn add_input(&self, s: String) {
        self.provided_inputs.borrow_mut().push_back(s);
    }
    pub fn show_all(&self) {
        for s in self.outputs.borrow().iter() {
            println!("{}", s)
        }
    }
}
#[cfg(test)]
impl InnerConsole for MockConsole {
    fn clear_screen(&self) {}
    fn get_input(&self) -> String {
        self.provided_inputs.borrow_mut().pop_front().unwrap()
    }
    fn show(&self, text: &str) {
        if SHOW_CONSOLE_OUTPUT {
            println!("{}", text);
        }
        self.outputs.borrow_mut().push(text.to_owned())
    }
}

#![cfg(test)]

use std::{cell::RefCell, collections::VecDeque};

use crate::ui::InnerConsole;

const SHOW_CONSOLE_OUTPUT: bool = false;

pub struct MockConsole {
    provided_inputs: RefCell<VecDeque<String>>,
    outputs: RefCell<Vec<String>>,
}

impl MockConsole {
    pub fn new() -> Self {
        Self {
            provided_inputs: RefCell::new(VecDeque::<String>::new()),
            outputs: RefCell::new(Vec::<String>::new()),
        }
    }
    pub fn add_input(&self, s: String) {
        println!("Input: {}", s);
        self.provided_inputs.borrow_mut().push_back(s);
    }
    pub fn add_inputs(&self, inputs: &[String]) {
        for input in inputs {
            self.add_input(input.to_string());
        }
    }

    pub fn show_all(&self) {
        for s in self.outputs.borrow().iter() {
            println!("{}", s)
        }
    }
}

impl InnerConsole for MockConsole {
    fn clear_screen(&self) {}
    fn get_input(&self) -> String {
        self.provided_inputs
            .borrow_mut()
            .pop_front()
            .expect("There should be inputs available")
    }
    fn show(&self, text: &str) {
        if SHOW_CONSOLE_OUTPUT {
            println!("{}", text);
        }
        self.outputs.borrow_mut().push(text.to_owned())
    }
}

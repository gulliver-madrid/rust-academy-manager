pub trait InnerConsole {
    fn clear_screen(&self);
    fn get_input(&self) -> String;
    fn show(&self, text: &str);
    fn close(&self); // Used for testing
}

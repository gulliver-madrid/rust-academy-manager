pub trait InnerConsole {
    fn clear_screen(&self);
    fn get_input(&self) -> String;
    fn mostrar(&self, texto: &str);
}
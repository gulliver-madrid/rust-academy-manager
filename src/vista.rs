use std::io;
pub struct Vista;

impl Vista {
    pub fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let input = String::from(input.trim());
        input
    }

    pub fn mostrar(&self, texto: &str) {
        println!("{}", texto);
    }
}

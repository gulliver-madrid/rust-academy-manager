use crate::{consola::Consola, repo::Repository};

pub struct Control{
    pub consola: Consola,
    pub repository: Repository
}

pub trait Component{
    fn render(&mut self, control: &Control);
}
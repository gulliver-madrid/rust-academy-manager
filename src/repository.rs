use crate::{
    dominio::{teachers::Profesores},
    persistencia::Persistencia,
};

pub struct Modelo {
    // profesores: Option<Profesores>
}

pub struct Repository {
    pub persistencia: Persistencia,
    pub modelo: Modelo,
}

impl Repository {
    #[allow(unused)]
    fn get_profesores(&mut self) -> Profesores {
        return self.persistencia.load_profesores();
    }
}

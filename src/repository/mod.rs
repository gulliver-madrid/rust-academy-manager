pub mod modelo;
pub mod persistencia;
mod repository;
mod serializable;
mod serialization;
pub use repository::create_repo;
pub use repository::Repository;
pub use persistencia::{Persistencia, PersistenciaTrait};

pub mod model;
mod persistence;
mod repository;
mod serializable;
mod serialization;
pub use persistence::{Persistence, PersistenceTrait};
pub use repository::create_repo;
pub use repository::Repository;

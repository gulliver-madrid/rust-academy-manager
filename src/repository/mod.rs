pub mod model;
mod persistence;
mod persistence_trait;
mod repository;
mod serializable;
mod serialization;

pub use persistence::JsonPersistence;
pub use persistence_trait::PersistenceTrait;
pub use repository::create_repo;
pub use repository::Repository;

pub mod model;
mod persistence;
mod persistence_trait;
mod repository;
mod serializable;
mod serialization;

pub use persistence::JsonPersistence;
pub use persistence_trait::PersistenceTrait;
pub use repository::create_repo_providing_model;
pub use repository::create_repository;
pub use repository::Repository;

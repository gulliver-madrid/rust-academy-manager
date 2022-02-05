mod add_teacher;
pub mod modelo;
pub mod persistencia;
mod remove_teacher;
mod repository;
mod serializable;
mod serialization;

pub use repository::Repository;

use crate::errors::SimpleError;
pub type SimpleResult = Result<(), SimpleError>;

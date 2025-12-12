use crate::{error::Error, path::Path};

pub mod error;
pub mod path;
pub mod prelude;

pub type Result<T> = std::result::Result<T, &'static dyn Error>;

pub trait Context {
    fn get<T>(&self, path: &Path) -> Result<T>;
}

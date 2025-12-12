mod not_found;

pub use not_found::*;

use crate::path::Path;

pub trait Error: std::error::Error {
    fn path(&self) -> &Path;
}

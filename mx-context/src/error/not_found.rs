use crate::path::Path;

#[derive(Debug, Clone)]
pub struct NotFoundError {
    path: Path,
}

impl From<Path> for NotFoundError {
    fn from(path: Path) -> Self {
        Self { path }
    }
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[mx::context::error] path '{}' not found", &self.path)
    }
}

impl std::error::Error for NotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl super::Error for NotFoundError {
    fn path(&self) -> &Path {
        &self.path
    }
}

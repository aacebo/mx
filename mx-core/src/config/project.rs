use std::{fs, path::Path};

use mx_error::{BadRequestError, DynError, NotFoundError, Result};
use serde::{Deserialize, Serialize};

///
/// `mx.yml`
///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: Option<String>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub plugins: Option<Vec<super::PluginConfig>>,
}

impl ProjectConfig {
    pub fn from_file(path: &Path) -> Result<Self> {
        let mut buf = path.to_path_buf();

        // default to project file
        if buf.is_dir() {
            buf.push("mx.yaml");
        } else if buf.file_name().unwrap() != "mx.yaml" {
            return Err(BadRequestError::from(format!(
                "invalid project filename '{}', expected 'mx.yaml'",
                buf.file_name().unwrap().display()
            ))
            .into());
        }

        if !buf.exists() {
            return Err(NotFoundError::from(format!("path '{}' not found", buf.display())).into());
        }

        let content = fs::read(&buf).expect(&format!("path '{}' not found", buf.display()));
        let value: Self = match serde_saphyr::from_slice(&content) {
            Err(err) => return Err(DynError::new(err).into()),
            Ok(v) => v,
        };

        Ok(value)
    }
}

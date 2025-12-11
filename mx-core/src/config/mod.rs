mod plugin;

pub use plugin::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: Option<String>,
    pub plugins: Vec<PluginConfig>,
}

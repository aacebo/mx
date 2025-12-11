mod plugin;

pub use plugin::*;

use serde::{Deserialize, Serialize};

///
/// `mx.yml`
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: Option<String>,
    pub plugins: Vec<PluginConfig>,
}

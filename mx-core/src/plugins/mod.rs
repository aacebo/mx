use crate::config;

pub trait Plugin {
    fn config(&self) -> config::PluginConfig;
}

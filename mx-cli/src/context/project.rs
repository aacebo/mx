use mx_core::config::ProjectConfig;

#[derive(Debug, Default, Clone)]
pub struct ProjectContext {
    config: ProjectConfig,
}

impl ProjectContext {
    pub fn config(&self) -> &ProjectConfig {
        &self.config
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProjectContextBuilder {
    config: Option<ProjectConfig>,
}

impl ProjectContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(&mut self, config: ProjectConfig) -> &mut Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> ProjectContext {
        ProjectContext {
            config: self.config.expect("project configuration is required"),
        }
    }
}
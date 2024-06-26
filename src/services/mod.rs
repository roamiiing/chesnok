use anyhow::Result;
use bollard::container;

use crate::context::Context;

pub mod app;
pub mod postgres;

pub enum ServiceKind {
    /// Service being developed with dploy
    App,
    Postgres,
    Keydb,
}

/// Env vars to expose to app service
pub trait EnvVars {
    fn env_vars(&self) -> Vec<(String, String)>;
}

pub struct ContainerConfig {
    container_name: String,
    image_name: String,
    config: container::Config<String>,
}

impl ContainerConfig {
    pub fn new(
        container_name: String,
        image_name: String,
        config: container::Config<String>,
    ) -> Self {
        Self {
            container_name,
            image_name,
            config,
        }
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    pub fn image_name(&self) -> &str {
        &self.image_name
    }

    pub fn config(&self) -> &container::Config<String> {
        &self.config
    }
}

pub trait ToContainerConfig {
    fn to_container_config(&self, context: &Context) -> Result<ContainerConfig>;
}

pub struct Services {
    app: Option<app::AppService>,
    postgres: Option<postgres::PostgresService>,
}

impl Services {
    pub fn from_context(context: &Context) -> Self {
        let mut app_service_env_vars = vec![];

        let postgres = postgres::PostgresService::from_context(context);

        if let Some(postgres) = &postgres {
            app_service_env_vars.extend(postgres.env_vars());
        }

        let app = context
            .should_create_app_service()
            .then(|| app::AppService::from_context(context, app_service_env_vars));

        Self { app, postgres }
    }

    pub fn to_container_configs(&self, context: &Context) -> Result<Vec<ContainerConfig>> {
        let mut configs = vec![];

        if let Some(postgres) = &self.postgres {
            configs.push(postgres.to_container_config(context)?);
        }

        Ok(configs)
    }

    pub fn env_vars(&self) -> Vec<(String, String)> {
        let mut env_vars = vec![];

        if let Some(postgres) = &self.postgres {
            env_vars.extend(postgres.env_vars());
        }

        env_vars
    }
}

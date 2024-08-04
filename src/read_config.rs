use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::{collections::HashMap, env};

#[derive(Clone, Debug, Deserialize)]
pub struct GatewayService {
    pub target_service: String,
    pub target_port: i32,
}

#[derive(Debug, Deserialize)]
pub struct GatewayConfig {
    pub services: HashMap<String, GatewayService>,
}

impl GatewayConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let cfg = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", app_env)).required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        cfg.try_deserialize()
    }
}

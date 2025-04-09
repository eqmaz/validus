#![allow(dead_code)]

// This is where we define our config structure and default values.
// We have a strictly defined structure for our config file.

use app_core::context::FeatureMapProvider;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub logging: LogConfig,

    #[serde(default)]
    pub features: HashMap<String, bool>,

    #[serde(default)]
    pub engine: EngineConfig,

    #[serde(default)]
    pub debug: bool,
}

/// Enables us to use the `features` field as a feature map.
impl FeatureMapProvider for AppConfig {
    fn feature_map(&self) -> &HashMap<String, bool> {
        &self.features
    }
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub output: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            output: "./logs/app.log".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    pub machine_id: u16,
}
impl Default for EngineConfig {
    fn default() -> Self {
        Self { machine_id: 101 }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            logging: LogConfig::default(),
            features: HashMap::new(),
            engine: Default::default(),
            debug: false,
        }
    }
}

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
    pub rest: RestConfig,

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
    pub output: String,
    pub level: String,
    pub format: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            output: "./logs/app.log".to_string(),
            level: "info".to_string(),
            format: "json".to_string(),
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

#[derive(Debug, Deserialize)]
pub struct RestConfig {
    pub bind_on: String,
}
impl Default for RestConfig {
    fn default() -> Self {
        Self {
            bind_on: "0.0.0.0:8080".to_string(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            logging: LogConfig::default(),
            features: HashMap::new(),
            engine: Default::default(),
            rest: Default::default(),
            debug: false,
        }
    }
}

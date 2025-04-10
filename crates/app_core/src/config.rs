use config::{Config as RawConfig, File, FileFormat};
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
/**
* @file app_config.rs
* @brief Configuration management for the application.
*
* This is a wrapper around the `config` crate.
* It supports loading from multiple paths and allows for easy access to the configuration values.
*
* - User-defined typed config struct
*
*/
use std::{any::Any, path::PathBuf, sync::Arc};

static CONFIG: OnceCell<Arc<dyn Any + Send + Sync>> = OnceCell::new();
static RAW: OnceCell<RawConfig> = OnceCell::new(); // For raw lookups like "foo.bar"

pub struct ConfigManager;

impl ConfigManager {
    /// Internal function: load and return (typed, raw)
    fn load_config<T>(search_paths: &[PathBuf], filename: &str) -> Result<(T, RawConfig), config::ConfigError>
    where
        T: DeserializeOwned,
    {
        let mut builder = RawConfig::builder();

        for path in search_paths {
            let file_path = path.join(filename);
            if file_path.exists() {
                builder = builder.add_source(File::from(file_path).format(FileFormat::Toml));
            }
        }

        let raw = builder.build()?;
        let typed = raw.clone().try_deserialize::<T>()?;
        Ok((typed, raw))
    }

    /// Initialize the config singleton. Should only be called once.
    pub fn init<T>(search_paths: &[PathBuf], filename: &str) -> Arc<T>
    where
        T: DeserializeOwned + Send + Sync + 'static,
    {
        let (typed, raw) =
            Self::load_config::<T>(search_paths, filename).unwrap_or_else(|e| panic!("Failed to load config: {}", e));

        let arc_config: Arc<dyn Any + Send + Sync> = Arc::new(typed);
        CONFIG.set(arc_config.clone()).expect("Config already initialized");
        RAW.set(raw).expect("Raw config already initialized");

        arc_config.downcast::<T>().expect("Type mismatch in config")
    }

    /// Get the globally initialized config as a typed Arc<T>
    pub fn get<T>() -> Arc<T>
    where
        T: Send + Sync + 'static,
    {
        CONFIG
            .get()
            .expect("Config not initialized")
            .clone()
            .downcast::<T>()
            .expect("Type mismatch in config")
    }

    /// Returns whether a dotted key exists in the raw config (e.g. "logging.level")
    pub fn has_key(key: &str) -> bool {
        RAW.get().expect("Config not initialized").get_string(key).is_ok()
    }

    /// Returns a string value from the config for a given dotted path (e.g. "logging.level")
    pub fn get_value(key: &str) -> Option<String> {
        RAW.get().and_then(|cfg| cfg.get_string(key).ok())
    }

    /// Returns an int value from the config for a given dotted path (e.g. "logging.level")
    pub fn get_int(key: &str) -> Option<i64> {
        RAW.get().and_then(|cfg| cfg.get_int(key).ok())
    }
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// Unit tests for config.rs
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;
    use tempfile::tempdir;

    #[derive(Debug, Deserialize)]
    struct MyAppConfig {
        logging: Logging,
    }

    #[derive(Debug, Deserialize)]
    struct Logging {
        level: String,
    }

    #[test]
    fn test_load_and_get_config() {
        // Prepare temp dir with config.toml
        let dir = tempdir().unwrap();
        let toml_path = dir.path().join("app.toml");
        fs::write(
            &toml_path,
            r#"
            [logging]
            level = "debug"
        "#,
        )
        .unwrap();

        let search_paths = vec![dir.path().to_path_buf()];

        // Load config
        ConfigManager::init::<MyAppConfig>(&search_paths, "app.toml");

        // Get typed access
        let cfg = ConfigManager::get::<MyAppConfig>();
        assert_eq!(cfg.logging.level, "debug");

        // Raw access
        assert!(ConfigManager::has_key("logging.level"));
        assert_eq!(ConfigManager::get_value("logging.level").unwrap(), "debug");

        // Should panic on re-init
        let result = std::panic::catch_unwind(|| {
            ConfigManager::init::<MyAppConfig>(&search_paths, "app.toml");
        });
        assert!(result.is_err());
    }
}

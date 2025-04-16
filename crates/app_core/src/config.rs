//! Application Config Manager
//!
//! Provides a reasonably flexible and testable configuration system:
//!
//! - App context will call `init_global_config::<T>()` once at startup to register the global config.
//! - Use `config::<T>()` to access the global config anywhere in the app (typed + raw).
//!     For example if your config struct is `AppConfig`, use `config::<AppConfig>()`.
//! - Use `ConfigManager::<T>::load()` to load a typed config + raw access (`your_config.toml`).
//! - Use `typed_config::<T>()` for only the typed config, or `raw_config()` for key-based lookups.
//!
//! **Global Access:**
//!   - Powered by `OnceCell`.
//!   - Acts like a singleton after calling `init_global_config`.
//!   - Enforced to be set only once (panic-safe).
//!
//! **Testability:**
//!   - Avoids global state in unit tests by calling `ConfigManager::<T>::load()` directly.
//!   - Uses the same API for raw lookups: `get_value()`, `get_bool()`, etc.

use crate::wout;
use config::{Config as RawConfig, File, FileFormat};
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use std::{any::Any, path::PathBuf, sync::Arc};

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// GLOBAL STORAGE (ONCE-CELL SINGLETONS)
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

static CONFIG: OnceCell<Arc<dyn Any + Send + Sync>> = OnceCell::new();
static RAW: OnceCell<RawConfig> = OnceCell::new();

/// Register the global configuration singleton.
/// This must be called exactly once at startup, usually from `AppContext::init_config`.
pub fn init_global_config<T>(search_paths: &[PathBuf], filename: &str)
where
    T: DeserializeOwned + Default + Send + Sync + 'static,
{
    let store = ConfigManager::<T>::load(search_paths, filename);

    CONFIG.set(store.typed.clone() as Arc<dyn Any + Send + Sync>).expect("Global config already initialized");

    RAW.set(store.raw).expect("Raw config already initialized");
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// CONFIG MANAGER STRUCT — LOADER + RAW ACCESS
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

#[derive(Debug)]
pub struct ConfigManager<T> {
    pub typed: Arc<T>,
    pub raw: RawConfig,
}

impl<T> ConfigManager<T>
where
    T: DeserializeOwned + Default + Send + Sync + 'static,
{
    /// Loads and deserializes the typed config from the given file.
    /// Falls back to default values if loading fails.
    pub fn load(search_paths: &[PathBuf], filename: &str) -> Self {
        let mut builder = RawConfig::builder();

        for path in search_paths {
            let file_path = path.join(filename);
            if file_path.exists() {
                //debug!("Found config at {:?}", file_path);
                builder = builder.add_source(File::from(file_path).format(FileFormat::Toml));
            }
            // else {
            //     debug!("Config file not found at {:?}", file_path);
            // }
        }

        match builder.build() {
            Ok(raw) => match raw.clone().try_deserialize::<T>() {
                Ok(typed) => {
                    //debug!("✔ Config deserialized.");
                    Self { typed: Arc::new(typed), raw }
                }
                Err(e) => {
                    wout!("Failed to parse config: {}", e);
                    wout!("Falling back to default config.");
                    Self { typed: Arc::new(T::default()), raw }
                }
            },
            Err(e) => {
                wout!("Config build failed: {}", e);
                wout!("Falling back to default config.");
                Self { typed: Arc::new(T::default()), raw: RawConfig::default() }
            }
        }
    }

    /// Check if a dotted key exists in the raw config.
    pub fn has_key(&self, key: &str) -> bool {
        self.raw.get_string(key).is_ok()
    }

    /// Get a string value by dotted key from raw config.
    pub fn get_value(&self, key: &str) -> Option<String> {
        self.raw.get_string(key).ok()
    }

    /// Get an int value by dotted key from raw config.
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.raw.get_int(key).ok()
    }

    /// Get a float value from the raw config by dotted key
    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.raw.get_float(key).ok()
    }

    /// Get a boolean value from the raw config by dotted key
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.raw.get_bool(key).ok()
    }
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// SINGLETON ACCESSORS — GENERIC GLOBAL ACCESS
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

pub fn config<T>() -> ConfigManager<T>
where
    T: Send + Sync + 'static,
{
    ConfigManager { typed: typed_config::<T>(), raw: raw_config().clone() }
}

/// Get the typed global config (must match `T` used in `init_config_global<T>()`)
pub fn typed_config<T>() -> Arc<T>
where
    T: Send + Sync + 'static,
{
    CONFIG
        .get()
        .expect("Global config not initialized")
        .clone()
        .downcast::<T>()
        .expect("Type mismatch in global config")
}

/// Get the raw config (used for key-based lookups)
pub fn raw_config() -> &'static RawConfig {
    RAW.get().expect("Raw config not initialized")
}

/// Returns true if the raw config contains a key (e.g. "logging.level")
pub fn config_has_key(key: &str) -> bool {
    RAW.get().map_or(false, |cfg| cfg.get_string(key).is_ok())
}

/// Gets a dotted string value from the raw config
pub fn config_value(key: &str) -> Option<String> {
    RAW.get().and_then(|cfg| cfg.get_string(key).ok())
}

/// Gets a string value from the raw config
pub fn config_string(key: &str) -> Option<String> {
    RAW.get().and_then(|cfg| cfg.get_string(key).ok())
}

/// Gets a dotted string value from the raw config
pub fn config_int(key: &str) -> Option<i64> {
    RAW.get().and_then(|cfg| cfg.get_int(key).ok())
}

/// Gets a dotted float value from the raw config
pub fn config_float(key: &str) -> Option<f64> {
    RAW.get().and_then(|cfg| cfg.get_float(key).ok())
}

/// Gets a boolean value from the raw config
pub fn config_bool(key: &str) -> Option<bool> {
    RAW.get().and_then(|cfg| cfg.get_bool(key).ok())
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// Unit tests for config module
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
#[cfg(test)]
mod tests {
    use super::*;
    use crate::console;
    use serde::Deserialize;
    use std::fs;
    use tempfile::tempdir;

    #[allow(dead_code)]
    #[derive(Debug, Deserialize, Default)]
    struct MyConfig {
        #[serde(default)]
        logging: Logging,
        #[serde(default)]
        features: std::collections::HashMap<String, bool>,
        #[serde(default)]
        debug: bool,
    }

    #[derive(Debug, Deserialize, Default)]
    struct Logging {
        #[serde(default)]
        level: String,
        #[serde(default)]
        file: String,
    }

    #[test]
    fn test_load_valid_config_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("app.toml");
        fs::write(
            &path,
            r#"
            debug = true
            [logging]
            level = "info"
            file = "/tmp/log.txt"
            [features]
            rest_api = true
        "#,
        )
        .unwrap();

        let config = ConfigManager::<MyConfig>::load(&[dir.path().to_path_buf()], "app.toml");

        assert_eq!(config.typed.debug, true);
        assert_eq!(config.typed.logging.level, "info");
        assert_eq!(config.typed.logging.file, "/tmp/log.txt");
        assert_eq!(config.typed.features.get("rest_api"), Some(&true));

        assert!(config.has_key("logging.level"));
        assert_eq!(config.get_value("logging.level").as_deref(), Some("info"));
        assert_eq!(config.get_bool("features.rest_api"), Some(true));
    }

    #[test]
    fn test_missing_config_file_falls_back_to_default() {
        let dir = tempdir().unwrap();
        let config = ConfigManager::<MyConfig>::load(&[dir.path().to_path_buf()], "missing.toml");

        assert_eq!(config.typed.debug, false);
        assert_eq!(config.typed.logging.level, "");
        assert_eq!(config.typed.logging.file, "");
        assert!(config.typed.features.is_empty());
    }

    #[test]
    fn test_invalid_toml_falls_back_to_default() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("app.toml");
        fs::write(&path, "not valid = toml:").unwrap();

        console::suspend(); // Avoid spamming console with warnings that we expect
        let config = ConfigManager::<MyConfig>::load(&[dir.path().to_path_buf()], "app.toml");
        console::resume();

        assert_eq!(config.typed.debug, false);
        assert_eq!(config.typed.logging.level, "");
    }

    #[test]
    fn test_get_int_and_float() {
        #[allow(dead_code)]
        #[derive(Debug, Deserialize, Default)]
        struct NumericConfig {
            #[serde(default)]
            threshold: i64,
            #[serde(default)]
            scale: f64,
        }

        let dir = tempdir().unwrap();
        let path = dir.path().join("num.toml");
        fs::write(
            &path,
            r#"
            threshold = 42
            scale = 1.5
        "#,
        )
        .unwrap();

        let config = ConfigManager::<NumericConfig>::load(&[dir.path().to_path_buf()], "num.toml");

        assert_eq!(config.get_int("threshold"), Some(42));
        assert_eq!(config.get_float("scale"), Some(1.5));
    }

    #[test]
    fn test_get_nonexistent_keys_return_none() {
        let config = ConfigManager::<MyConfig>::load(&[], "nonexistent.toml");
        assert_eq!(config.get_int("foo.bar"), None);
        assert_eq!(config.get_value("foo.bar"), None);
        assert_eq!(config.get_bool("foo.bar"), None);
        assert!(!config.has_key("foo.bar"));
    }

    #[test]
    fn test_singleton_access_once() {
        use std::sync::Once;

        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            let dir = tempdir().unwrap();
            let path = dir.path().join("app.toml");
            fs::write(
                &path,
                r#"
                debug = true
                [logging]
                level = "warn"
                file = "global.log"
            "#,
            )
            .unwrap();

            init_global_config::<MyConfig>(&[dir.path().to_path_buf()], "app.toml");

            // Validate typed global access
            let typed = typed_config::<MyConfig>();
            assert_eq!(typed.logging.level, "warn");
            assert_eq!(typed.debug, true);

            // Validate raw access
            assert_eq!(config_string("logging.level").as_deref(), Some("warn"));
            assert_eq!(config_bool("debug"), Some(true));
        });
    }
}

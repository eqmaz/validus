// crates/app_core/tests/config_tests.rs

use std::{fs, path::PathBuf, sync::Once};

use app_core::config::{init_global_config, typed_config, ConfigManager};
use serde::Deserialize;
use tempfile::tempdir;

// === Example config struct -----

#[derive(Debug, Deserialize, Default)]
struct TestConfig {
    #[serde(default)]
    logging: Logging,

    #[serde(default)]
    database: Database,
}

#[derive(Debug, Deserialize, Default)]
struct Logging {
    #[serde(default = "default_log_level")]
    level: String,

    #[serde(default = "default_log_output")]
    output: String,
}

#[derive(Debug, Deserialize, Default)]
struct Database {
    #[serde(default = "default_db_host")]
    host: String,

    #[serde(default = "default_db_port")]
    port: u16,
}

// === A few helper methods -----

// Default value providers
fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_output() -> String {
    "stdout".to_string()
}
fn default_db_host() -> String {
    "127.0.0.1".to_string()
}
fn default_db_port() -> u16 {
    5432
}

// Used to prevent global config from being initialized more than once in this test file
static INIT: Once = Once::new();

/// Create a temporary TOML config file and return the containing directory
/// This assumes we have write permissions in the current directory
fn setup_temp_config() -> (PathBuf, PathBuf) {
    let dir = tempdir().unwrap();

    // Check if we have write permissions for this directory
    if !dir.path().is_dir() {
        panic!("Integration test failed to create temporary directory for config file");
        // TODO - probably handle this more gracefully later
    }

    let file_path = dir.path().join("config.toml");

    fs::write(
        &file_path,
        r#"
        [logging]
        level = "warn"

        [database]
        host = "db.example.com"
    "#,
    )
    .unwrap();

    (dir.into_path(), file_path)
}

#[test]
fn test_load_toml_with_config_manager() {
    let (dir, _) = setup_temp_config();

    let config = ConfigManager::<TestConfig>::load(&[dir.clone()], "config.toml");

    assert_eq!(config.typed.logging.level, "warn");
    assert_eq!(config.typed.logging.output, "stdout"); // default
    assert_eq!(config.typed.database.host, "db.example.com");
    assert_eq!(config.typed.database.port, 5432); // default
}

#[test]
fn test_global_config_initialization_and_access() {
    let (dir, _) = setup_temp_config();

    INIT.call_once(|| {
        init_global_config::<TestConfig>(&[dir.clone()], "config.toml");

        let config = typed_config::<TestConfig>();
        assert_eq!(config.logging.level, "warn");
        assert_eq!(config.database.host, "db.example.com");
    });
}

#[test]
#[should_panic(expected = "Global config already initialized")]
fn test_global_config_reinitialization_panics() {
    let (dir, _) = setup_temp_config();

    INIT.call_once(|| {
        init_global_config::<TestConfig>(&[dir.clone()], "config.toml");
    });

    // This should panic since the global config is already set
    init_global_config::<TestConfig>(&[dir], "config.toml");
}

#[test]
#[should_panic(expected = "Type mismatch in global config")]
fn test_global_config_type_mismatch_panics() {
    let (dir, _) = setup_temp_config();

    INIT.call_once(|| {
        init_global_config::<TestConfig>(&[dir.clone()], "config.toml");
    });

    #[derive(Debug, Deserialize)]
    struct WrongType;

    // This should panic due to a type mismatch
    let _ = typed_config::<WrongType>();
}

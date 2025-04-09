use std::fs;
use std::path::PathBuf;
use std::sync::Once;

use app_core::config::ConfigManager;
use serde::Deserialize;
use tempfile::tempdir;

#[derive(Debug, Deserialize)]
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

// Ensures tests don't re-init the global CONFIG more than once
static INIT: Once = Once::new();

fn setup_temp_config() -> PathBuf {
    let tmp_dir = PathBuf::from("tests/tmp_config");
    let file_path = tmp_dir.join("config.toml");

    fs::create_dir_all(&tmp_dir).unwrap();

    let toml_data = r#"
[logging]
level = "warn"

[database]
host = "db.example.com"
    "#;

    fs::write(&file_path, toml_data).unwrap();
    file_path
}

#[test]
fn test_loading_real_toml_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("settings.toml");

    fs::write(
        &file_path,
        r#"
        [database]
        url = "db.example.com"
    "#,
    )
    .unwrap();

    let paths = vec![dir.path().to_path_buf()];
    let config = ConfigManager::init::<TestConfig>(&paths, "settings.toml");
    assert_eq!(config.database.host, "db.example.com");
}

#[test]
#[should_panic(expected = "Config already initialized")]
fn test_reinitialization_panics() {
    let path = setup_temp_config();
    let dir = path.parent().unwrap().to_path_buf();

    // This will panic if init() was already called
    ConfigManager::init::<TestConfig>(&[dir], "config.toml");
}

#[test]
#[should_panic(expected = "Type mismatch in config")]
fn test_get_wrong_type_panics() {
    let path = setup_temp_config();
    let dir = path.parent().unwrap().to_path_buf();
    ConfigManager::init::<TestConfig>(&[dir], "config.toml");

    #[derive(Debug, Deserialize)]
    struct WrongType;
    let _ = ConfigManager::get::<WrongType>();
}

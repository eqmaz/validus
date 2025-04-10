mod app_config;
mod app_entry;
mod app_errors;
mod service;
mod state;

use app_config::AppConfig;
use app_core::prelude::*;
use std::path::PathBuf;

#[macro_use]
extern crate app_core;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Constants for config and logging
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const LOG_STREAM: &'static str = "./logs/app.log";
const LOG_LEVEL: &'static str = "debug";
const CFG_FILE: &'static str = "config.toml";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Application bootstrap. See app_entry.rs for business logic.
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn main() {
    // Config file search paths, relative to the executable's location
    let config_paths = vec![
        PathBuf::from("./config"), //PathBuf::from(".")
    ];

    // Build AppInitOptions
    // TODO get logger settings from config, use params as fallback
    let opts = AppInitOptions::new()
        .with_config(config_paths, CFG_FILE)
        .with_logger(LOG_STREAM, LOG_LEVEL);

    // Initialize the application context (Config struct in app_config.rs)
    let app = AppContext::init::<AppConfig>(opts);

    // Parse the config file for feature flags
    app.extract_feature_flags::<AppConfig>();

    // Start the business logic
    app.start(app_entry::run)
}

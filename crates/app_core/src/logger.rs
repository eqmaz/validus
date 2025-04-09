/*!
# App Logger

A lightweight, structured JSON logger for use within the `app_core` crate and the wider application.

## Features

- Global thread-safe logger instance (`Logger`)
- Support for contextual loggers (`LoggerInstance`) with default fields (e.g. `request_id`, `user_id`)
- JSON log lines with stable field ordering
- Optional log levels: `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`
- Custom `kind` fields (e.g. `success`, `critical`) for enriched semantic logging
- Output fields: `time`, `lvl`, `msg`, `fields`

## Example Output
    {
      "time": "2025-04-05T21:55:12.111Z",
      "lvl": "INFO",
      "msg": "User registered",
      "fields": {
        "kind": "success",
        "user_id": 42,
        "region": "eu-west"
      }
    }

## Usage
    1. Initialize global logger
        Logger::init("./logs/app.log", "info");
        Logger::info("Server started", None);

    2. Use a contextual logger with default fields
        let logger = Logger::new_instance()
            .with_field("user_id", json!(42))
            .with_field("request_id", json!("abc-123"));
        logger.success("User registered", None);
        logger.warn("Quota exceeded", Some(&[("quota", json!(100))]));
*/

use chrono::Utc;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    sync::Mutex,
};

/// Internal log level representation used for filtering.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum LogLevel {
    Trace = 0,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Parse log level from string (e.g. "info") into enum.
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }
}

// Shared file handle for writing logs
lazy_static! {
    static ref LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
    static ref LOG_PATH: Mutex<Option<String>> = Mutex::new(None);
    static ref MIN_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Info);
}

/// Global logger for the entire application.
pub struct Logger;

impl Logger {
    /// Initializes the global logger. Must be called once at application startup.
    ///
    /// `path` is the path to the log file.
    /// `min_level` is the minimum log level to write (`"info"`, `"debug"`, etc.).
    pub fn init<P: AsRef<Path>>(path: P, min_level: &str) {
        let path_str = path.as_ref().to_string_lossy().to_string();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("Failed to open log file");

        *LOG_FILE.lock().unwrap() = Some(file);
        *LOG_PATH.lock().unwrap() = Some(path_str);
        *MIN_LEVEL.lock().unwrap() = LogLevel::from_str(min_level);
    }

    /// Create a new instance of a contextual logger that can include default fields.
    pub fn new_instance() -> LoggerInstance {
        LoggerInstance {
            default_fields: IndexMap::new(),
        }
    }

    /// Internal implementation used by both global and instance loggers.
    fn log(
        level: LogLevel,
        level_str: &str,
        message: &str,
        fields: Option<&[(&str, Value)]>,
        kind: Option<&str>,
        extra: Option<&IndexMap<String, Value>>,
    ) {
        if level < *MIN_LEVEL.lock().unwrap() {
            return;
        }

        let now = Utc::now().to_rfc3339();

        // All non-core fields go here
        let mut field_map: IndexMap<String, Value> = IndexMap::new();

        if let Some(kind) = kind {
            field_map.insert("kind".to_string(), json!(kind));
        }

        if let Some(extra_fields) = fields {
            for (k, v) in extra_fields {
                field_map.insert(k.to_string(), v.clone());
            }
        }

        if let Some(default_fields) = extra {
            for (k, v) in default_fields {
                // Only insert if not already provided
                field_map.entry(k.clone()).or_insert_with(|| v.clone());
            }
        }

        // Compose full JSON log entry
        let mut log: IndexMap<String, Value> = IndexMap::new();
        log.insert("time".to_string(), json!(now));
        log.insert("lvl".to_string(), json!(level_str));
        log.insert("msg".to_string(), json!(message));

        if !field_map.is_empty() {
            log.insert(
                "fields".to_string(),
                serde_json::to_value(field_map).unwrap(),
            );
        }

        let json_line = serde_json::to_string(&log).unwrap();

        if let Some(ref mut file) = *LOG_FILE.lock().unwrap() {
            writeln!(file, "{json_line}").ok();
        }
    }

    /// Gets the log file or output stream path
    pub fn log_destination() -> Option<String> {
        LOG_PATH.lock().ok()?.clone()
    }

    // === Public global logging functions (no context) -----

    pub fn trace(msg: &str, fields: Option<&[(&str, Value)]>) {
        Self::log(LogLevel::Trace, "TRACE", msg, fields, None, None);
    }

    pub fn debug(msg: &str, fields: Option<&[(&str, Value)]>) {
        Self::log(LogLevel::Debug, "DEBUG", msg, fields, None, None);
    }

    pub fn info(msg: &str, fields: Option<&[(&str, Value)]>) {
        Self::log(LogLevel::Info, "INFO", msg, fields, None, None);
    }

    pub fn success(msg: &str, fields: Option<&[(&str, Value)]>) {
        Self::log(LogLevel::Info, "INFO", msg, fields, Some("success"), None);
    }

    pub fn warn(msg: &str, fields: Option<&[(&str, Value)]>) {
        Self::log(LogLevel::Warn, "WARN", msg, fields, None, None);
    }

    pub fn error(msg: &str, fields: Option<&[(&str, Value)]>) {
        Self::log(LogLevel::Error, "ERROR", msg, fields, None, None);
    }

    pub fn critical(msg: &str, fields: Option<&[(&str, Value)]>) {
        Self::log(
            LogLevel::Error,
            "ERROR",
            msg,
            fields,
            Some("critical"),
            None,
        );
    }
}

/// Contextual logger that carries persistent fields across all logs it emits.
/// It uses the same underlying logging mechanism as the global logger.
#[derive(Clone)]
pub struct LoggerInstance {
    default_fields: IndexMap<String, Value>,
}

impl LoggerInstance {
    /// Add a single default field (e.g. `request_id`) to this logger instance.
    pub fn with_field(mut self, key: &str, value: Value) -> Self {
        self.default_fields.insert(key.to_string(), value);
        self
    }

    /// Add multiple default fields to this logger instance.
    pub fn with_fields(mut self, fields: &[(&str, Value)]) -> Self {
        for (k, v) in fields {
            self.default_fields.insert((*k).to_string(), v.clone());
        }
        self
    }

    /// Internal log call — delegates to the global Logger with its own fields merged in.
    fn log(
        &self,
        level: LogLevel,
        level_str: &str,
        msg: &str,
        fields: Option<&[(&str, Value)]>,
        kind: Option<&str>,
    ) {
        Logger::log(
            level,
            level_str,
            msg,
            fields,
            kind,
            Some(&self.default_fields),
        );
    }

    // === Instance logging methods — uses its own default fields -----

    pub fn trace(&self, msg: &str, fields: Option<&[(&str, Value)]>) {
        self.log(LogLevel::Trace, "TRACE", msg, fields, None);
    }

    pub fn debug(&self, msg: &str, fields: Option<&[(&str, Value)]>) {
        self.log(LogLevel::Debug, "DEBUG", msg, fields, None);
    }

    pub fn info(&self, msg: &str, fields: Option<&[(&str, Value)]>) {
        self.log(LogLevel::Info, "INFO", msg, fields, None);
    }

    pub fn success(&self, msg: &str, fields: Option<&[(&str, Value)]>) {
        self.log(LogLevel::Info, "INFO", msg, fields, Some("success"));
    }

    pub fn warn(&self, msg: &str, fields: Option<&[(&str, Value)]>) {
        self.log(LogLevel::Warn, "WARN", msg, fields, None);
    }

    pub fn error(&self, msg: &str, fields: Option<&[(&str, Value)]>) {
        self.log(LogLevel::Error, "ERROR", msg, fields, None);
    }

    pub fn critical(&self, msg: &str, fields: Option<&[(&str, Value)]>) {
        self.log(LogLevel::Error, "ERROR", msg, fields, Some("critical"));
    }
}

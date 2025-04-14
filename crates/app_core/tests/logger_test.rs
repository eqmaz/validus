use app_core::logger::{Logger};
use serde_json::{json, Value};
use std::{
    fs::{File},
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Once,
};
use tempfile::tempdir;

static INIT: Once = Once::new();

fn init_logger_once(path: &PathBuf) {
    INIT.call_once(|| {
        Logger::init(path, "info");
    });
}

fn read_log_lines(path: &PathBuf) -> Vec<Value> {
    let file = File::open(path).expect("Log file not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(&line.unwrap()).ok())
        .collect()
}


#[test]
fn test_global_logger_output() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("app.log");

    // Call directly — do NOT use `init_logger_once` here!
    Logger::init(&file_path, "info");

    Logger::info("Server started", None);
    Logger::warn("High memory usage", Some(&[("usage", json!(92.5))]));
    Logger::success("Task completed", Some(&[("task_id", json!("abc-123"))]));

    // flush before reading file
    Logger::flush();

    let logs = read_log_lines(&file_path);
    assert!(
        logs.len() >= 3,
        "Expected at least 3 logs, got {} — logs: {:#?}",
        logs.len(),
        logs
    );

    let last = logs.last().unwrap();
    assert_eq!(last["msg"], "Task completed");
    assert_eq!(last["lvl"], "INFO");
    assert_eq!(last["fields"]["kind"], "success");
    assert_eq!(last["fields"]["task_id"], "abc-123");
}


#[test]
fn test_contextual_logger_output() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("contextual.log");

    init_logger_once(&file_path);

    let logger = Logger::new_instance()
        .with_field("user_id", json!(42))
        .with_field("region", json!("eu-west"));

    logger.info("User logged in", None);
    logger.critical("Token expired", Some(&[("token_id", json!("tok-xyz"))]));

    let logs = read_log_lines(&file_path);
    assert!(logs.len() >= 2);

    let last = logs.last().unwrap();
    assert_eq!(last["msg"], "Token expired");
    assert_eq!(last["lvl"], "ERROR");
    assert_eq!(
        last["fields"]["kind"],
        Value::String("critical".to_string())
    );
    assert_eq!(
        last["fields"]["user_id"],
        Value::Number(42.into())
    );
    assert_eq!(
        last["fields"]["region"],
        Value::String("eu-west".to_string())
    );
    assert_eq!(
        last["fields"]["token_id"],
        Value::String("tok-xyz".to_string())
    );
}

#[test]
fn test_log_level_filtering() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("filtered.log");

    Logger::init(&file_path, "warn"); // Only allow warn or higher

    Logger::debug("This should NOT appear", None);
    Logger::info("This should NOT appear", None);
    Logger::warn("This should appear", None);
    Logger::error("Also appears", None);

    let logs = read_log_lines(&file_path);
    assert!(logs.len() == 2);
    assert_eq!(logs[0]["lvl"], "WARN");
    assert_eq!(logs[1]["lvl"], "ERROR");
}


//! Rich, extensible error handling for application and framework use.
//!
//! This module defines the `AppError` type — a highly structured, ergonomic,
//! and extensible error core with:
//!
//! - Classification via string-based `kind` (e.g. "auth", "db", etc.)
//! - Mandatory error codes (`Cow<'static, str>`)
//! - Human-readable messages
//! - Optional tags (`Vec<String>`) for context or filtering
//! - Structured metadata via `serde_json::Value`
//! - Built-in backtrace capture
//! - Error source chaining (`previous`)
//!
//! Includes the `app_err!` macro for concise ergonomic creation, and the
//! `IntoAppError` trait for automatic promotion of other error types.
//!
//! ## Example
//! ```rust
//! use app_core::app_err;
//!
//! let err = app_err!(
//!     "E401",
//!     "Unauthorized access",
//!     tags: ["auth", "user"],
//!     data: { "user_id" => 42, "ip" => "127.0.0.1" }
//! ).with_kind("auth");
//! ```

use crate::logger;
use serde_json::Value;
use std::any::Any;
use std::backtrace::Backtrace;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub trait ErrorCode {
    fn code(&self) -> &'static str;
    fn format(&self) -> &'static str;

    /// Optional default kind for this code
    fn kind(&self) -> &'static str {
        "generic"
    }
}

/// Core application error type used across the framework and userland code.
#[derive(Debug)]
pub struct AppError {
    /// String-based error kind (e.g. "auth", "db", "internal").
    pub kind: Cow<'static, str>,

    /// Required application-specific error code (e.g. `"E404"`).
    pub code: Cow<'static, str>,

    /// Human-readable message describing the error.
    pub message: Cow<'static, str>,

    /// Optional classification tags for context.
    pub tags: Vec<String>,

    /// Arbitrary metadata (numbers, booleans, strings, JSON, etc.).
    pub data: HashMap<String, Value>,

    /// Captured backtrace from point of error creation.
    pub backtrace: Backtrace,

    /// Optional previous error in the chain.
    pub previous: Option<Box<dyn Error + Send + Sync>>,
}

impl AppError {
    /// Creates a new `AppError` with a code and message.
    pub fn new<C: Into<Cow<'static, str>>, M: Into<Cow<'static, str>>>(code: C, message: M) -> Self {
        Self {
            kind: Cow::Borrowed("generic"),
            code: code.into(),
            message: message.into(),
            tags: vec![],
            data: HashMap::new(),
            backtrace: Backtrace::capture(),
            previous: None,
        }
    }

    /// Promotes any error into an `AppError`.
    pub fn from_error<E>(err: E) -> Self
    where
        E: Error + Send + Sync + 'static + Any,
    {
        if let Some(app_err) = (&err as &dyn Any).downcast_ref::<AppError>() {
            return Self {
                kind: app_err.kind.clone(),
                code: app_err.code.clone(),
                message: app_err.message.clone(),
                tags: app_err.tags.clone(),
                data: app_err.data.clone(),
                backtrace: Backtrace::capture(),
                previous: Some(Box::new(err)),
            };
        }

        Self {
            kind: Cow::Borrowed("generic"),
            code: Cow::Borrowed("undefined"),
            message: Cow::Owned(err.to_string()),
            tags: vec![],
            data: HashMap::new(),
            backtrace: Backtrace::capture(),
            previous: Some(Box::new(err)),
        }
    }

    /// Construct from typed error code + format template variables.
    ///
    /// Unused keys in the data are ignored.
    /// Missing keys in the template are replaced with `""`.
    pub fn from_code<C: ErrorCode>(code: C, data: Value) -> Self {
        let mut message = code.format().to_string();

        // Collect all {placeholders} first to avoid borrow issues
        let keys: Vec<String> = {
            let mut ks = vec![];
            let mut rest = message.as_str();
            while let Some(start) = rest.find('{') {
                if let Some(end) = rest[start + 1..].find('}') {
                    let key = &rest[start + 1..start + 1 + end];
                    ks.push(key.to_string());
                    rest = &rest[start + end + 2..]; // Skip past this placeholder
                } else {
                    break;
                }
            }
            ks
        };

        // Replace placeholders with values
        if let Some(obj) = data.as_object() {
            for key in keys {
                let val = obj
                    .get(&key)
                    .map(|v| match v {
                        Value::String(s) => s.clone(),
                        other => other.to_string(),
                    })
                    .unwrap_or_default();
                message = message.replace(&format!("{{{}}}", key), &val);
            }
        }

        let mut err = AppError::new(code.code(), message).with_kind(code.kind());

        // Add all data fields to the metadata
        if let Some(obj) = data.as_object() {
            for (k, v) in obj {
                err = err.with_data(k, v.clone());
            }
        }

        err
    }

    /// Assigns a kind string for this error.
    pub fn with_kind(mut self, kind: &str) -> Self {
        self.kind = Cow::Owned(kind.to_string());
        self
    }

    /// Override or update the error code.
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Cow::Owned(code.to_string());
        self
    }

    /// Add a tag for categorization.
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Add multiple tags for categorization.
    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        for tag in tags {
            self.tags.push(tag.to_string());
        }
        self
    }

    /// Add structured metadata.
    pub fn with_data(mut self, key: &str, value: Value) -> Self {
        self.data.insert(key.to_string(), value);
        self
    }

    /// Attach a previous error for chaining.
    pub fn with_previous<E>(mut self, err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.previous = Some(Box::new(err));
        self
    }

    /// Returns the kind string (e.g. `"auth"`).
    pub fn kind_str(&self) -> &str {
        &self.kind
    }

    /// Returns a formatted debug backtrace string.
    pub fn trace(&self) -> String {
        format!("{:?}", self.backtrace)
    }

    /// Returns the deepest error in the chain.
    pub fn root_cause(&self) -> &dyn Error {
        let mut cause = self as &dyn Error;
        while let Some(next) = cause.source() {
            cause = next;
        }
        cause
    }

    /// Log the error using the global logger instance.
    pub fn log(&self) {
        // Add kind to fields
        let mut fields: Vec<(&str, Value)> = vec![("kind", serde_json::json!(self.kind_str()))];

        // Add tags as a JSON array
        if !self.tags.is_empty() {
            fields.push(("tags", serde_json::json!(self.tags)));
        }

        // Add all metadata fields
        for (k, v) in &self.data {
            fields.push((k.as_str(), v.clone()));
        }

        // Step 4: Send to logger
        logger::Logger::error(&self.message, Some(&fields));
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} ({})", self.code, self.message, self.kind)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.previous.as_ref().map(|e| &**e as &(dyn Error + 'static))
    }
}

/// Trait extension to convert any error into an `AppError` with `.appify()`.
pub trait IntoAppError {
    fn appify(self) -> AppError;
}

impl<E> IntoAppError for E
where
    E: Error + Send + Sync + 'static + Any,
{
    fn appify(self) -> AppError {
        AppError::from_error(self)
    }
}

/// Macro for ergonomic error creation.
///
/// ### Basic usage
/// ```rust
/// app_err!("E400", "Bad request")
/// ```
///
/// ### With tags
/// ```rust
/// app_err!("E401", "Unauthorized", tags: ["auth", "token"])
/// ```
///
/// ### With tags and structured data
/// ```rust
/// app_err!("E500", "Failure", tags: ["internal"], data: {
///     "retry" => 3,
///     "timeout" => "10s"
/// })
/// ```
#[macro_export]
macro_rules! app_err {
    // Minimal (code, msg)
    ($code:expr, $msg:expr) => {{
        $crate::error::AppError::new($code, $msg)
    }};

    // With tags
    ($code:expr, $msg:expr, tags: [$($tag:expr),*]) => {{
        $crate::error::AppError {
            tags: vec![$($tag.into()),*],
            ..$crate::error::AppError::new($code, $msg)
        }
    }};

    // With tags + data
    ($code:expr, $msg:expr, tags: [$($tag:expr),*], data: { $($key:expr => $val:expr),* }) => {{
        $crate::error::AppError {
            tags: vec![$($tag.into()),*],
            data: {
                let mut m = ::std::collections::HashMap::new();
                $(m.insert($key.into(), ::serde_json::json!($val));)*
                m
            },
            ..$crate::error::AppError::new($code, $msg)
        }
    }};
}

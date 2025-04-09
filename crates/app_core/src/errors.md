# `app_core::error` — AppError System

Structured, extensible application error type with:
- Mandatory error codes
- Human-readable messages
- Optional tags
- Structured metadata
- Backtrace capture
- Error chaining
- User-defined error kinds
- Macros for ergonomic usage

---

## Usage examples

```rust
use app_core::error::*;
use app_core::app_err;
use serde_json::json;

/// Creating error directly
let err = AppError::new("E100", "Something went wrong");

/// Adding meta tags
let err = AppError::new("E101", "DB write failed")
    .with_tag("db")
    .with_tag("insert")
    .with_data("record_id", json!(42))
    .with_data("collection", json!("users"));

/// Attaching previous errors
let io_err = std::fs::File::open("missing.txt").unwrap_err();
let err = AppError::new("E102", "Failed to read config")
    .with_previous(io_err);
    
/// Promoting a normal rust error
fn read_file() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File missing"))
}
let err = read_file().map_err(AppError::from_error)?;
/// Or use .appify() trait sugar:
let err = std::env::var("MY_VAR").unwrap_err().appify();
  
    
/// Using the app_err! macro
let err = app_err!("E400", "Bad request");
let err = app_err!("E401", "Unauthorized", tags: ["auth", "user"]);
let err = app_err!(
    "E500",
    "Internal failure",
    tags: ["internal", "panic"],
    data: {
        "trace_id" => "abc-123",
        "retries" => 2,
        "env" => "production"
    }
);

/// Printing and traces
println!("Error: {err}");
println!("Kind: {}", err.kind_str());
println!("Trace:\n{}", err.trace());
println!("Root cause: {}", err.root_cause());
  

/// Defining custom error kinds
#[derive(Debug)]
enum MyErrorKind {
    Auth,
    Validation,
    ExternalService,
}

impl ErrorKind for MyErrorKind {
    fn as_str(&self) -> &str {
        match self {
            MyErrorKind::Auth => "auth",
            MyErrorKind::Validation => "validation",
            MyErrorKind::ExternalService => "external_service",
        }
    }
}

let err = AppError::new("E103", "Invalid credentials")
    .with_kind(MyErrorKind::Auth);
```




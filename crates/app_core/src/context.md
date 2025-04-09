# AppContext design

This is basically the app lifecycle manager and future DI container

## Features:
- Thread-safe struct
- Initializers for logger and config
- Generic AppInitOptions<T> with logger and config injection
- Feature flag system, FeatureMapProvider trait. Can auto load feature flags from config
- Signal handling (SIGTERM, SIGINT)
- Graceful shutdown with Lifecycle hooks

# Structs

### `AppContext`
Manages lifecycle, flags, and graceful shutdown.  
Typically constructed via `AppContext::init(...)` and passed through the app.

### `AppInitOptions<T>`
Used to configure logging and config paths before bootstrapping.

### `AppLoggerOptions`
Lightweight holder for log path and minimum level.

### `AppConfigOptions<T>`
Specifies config search paths and filename.  
Used internally during app bootstrapping.

## Trait: `FeatureMapProvider`
To allow your config struct to auto-load feature flags from `[features]` section:
```rust
pub trait FeatureMapProvider {
    fn feature_map(&self) -> &HashMap<String, bool>;
}
impl FeatureMapProvider for AppConfig {
    fn feature_map(&self) -> &HashMap<String, bool> {
        &self.features
    }
}
```

## Usage
### Create AppInitOptions
```rust
// List of paths to search for config file relative to the exe
let config_paths = vec![
    PathBuf::from("./config"),
    PathBuf::from("."),
];

// Building the options struct to init the App with
let opts = AppInitOptions::new()
    .with_config(config_paths, "config.toml")
    .with_logger("./logs/app.log", "debug");
```
### Initialize AppContext
```rust
let app = AppContext::init::<AppConfig>(opts)
    .with_feature_flags(HashMap::from([
        ("manual_feature".into(), true),
    ]))
    .extract_feature_flags::<AppConfig>();
```

### Run your app
```rust
app.start(|app| {
    sout!("App started");

    if app.feature_enabled("dev_mode") {
        sout!("Developer mode enabled");
    }

    app.on_shutdown(|| {
        sout!("Cleanup logic before shutdown...");
    });

    // Business logic entry point here
});
```
## Runtime Feature Flags
You can inject them manually like this
```rust
app.with_feature_flags(HashMap::from([
    ("foo".into(), true),
    ("bar".into(), false),
]));
```

Or load them from a config file like this
```toml
[features]
foo = true
bar = false
```
and then load them with
```rust
app.extract_feature_flags::<AppConfig>();
```
## Shutdown hooks
```rust
app.on_shutdown(|| {
    sout!("Running shutdown logic...");
});
```
## Signal handling
On startup, AppContext installs a ctrlc::set_handler() hook.

If SIGINT (Ctrl+C) or SIGTERM is received, it:
- Logs a warning
- Triggers graceful shutdown after start() completes

## Thread Safety
All internal state-like feature flags and shutdown hooks are guarded
with Arc<Mutex<...>> so they're thread-safe and cloneable if needed. <br>
AppContext itself is **not** Clone by default).

## TO DO
- Panic recovery and centralized error handling
- Dependency injection container
- Background task scheduling
- Health checks / runtime probes
- Structured tracing like OpenTelemetry / tracing crate



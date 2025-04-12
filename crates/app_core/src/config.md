# Config Module

## How the config system works within app_core

- **Works seamlessly with `AppContext`**
- **Typed + raw access to config values**
- **Global access via `OnceCell` singleton**
- **Test-friendly instances (no global side effects)**

---

## 🧠 How It Works

This is a a dual layer configuration system:

| Layer         | Purpose                                       | Example Usage                      |
|---------------|-----------------------------------------------|------------------------------------|
| **Typed**     | Deserializes into userland `AppConfig` struct | `config::<AppConfig>().typed.foo` |
| **Raw**       | Key-based lookups (`config["foo.bar"]`)       | `config_value("logging.level")`   |

### 🔁 Global Singleton

At startup, the global config is initialized like this:

```rust
// AppContext::init() will call this under the hood
init_global_config::<YourAppConfigStruct>(&search_paths, "app.toml");
```

Search paths is just a vector of directories where the config file may be located.  Internally, this loads and registers the config using OnceCell.

Once this is done, you can access config from anywhere in the app via:

### EITHER access via `config::<T>()`
```rust
use app_core::config::config;

let cfg = config::<AppConfig>();
let level = cfg.get_value("logging.level");

// or even
cfg.typed.logging.level.clone()
```

### Or use raw accessors directly:

```rust
use app_core::config::config_string;

let log_level = config_string("logging.level").unwrap_or("info".into());
```

### AppContext Integration

AppContext sets up config in init() automatically:

```rust
pub fn init<T>(opts: AppInitOptions<T>) -> Self
where
    T: DeserializeOwned + Default + Send + Sync + 'static,
{
    if let Some(cfg) = &opts.config {
        init_global_config::<T>(&cfg.search_paths, &cfg.file_name);
    }

    // ...
}
```

This means that as long as the main app registers the config, everything else can freely call `config::<T>()`.

### Testing With Local Config

You don’t have to touch the global singleton in unit tests. Instead:

```rust
let manager = ConfigManager::<MyTestConfig>::load(&[path], "app.toml");

assert_eq!(manager.get_value("logging.level").unwrap(), "debug");
```

---

## Accessing Config Anywhere

### Typed Access

```rust
let typed = config::<AppConfig>().typed;
let debug = typed.debug;
```

### Raw Key-Based Access

```rust
use app_core::config::*;

let level = config_value("logging.level");
let enabled = config_bool("features.rest_api");
let id = config_int("engine.machine_id");
```

### In Structs Without Generics

If you're in a place like `TradeEngine::new()`:

```rust
use app_core::config::config_int;

let machine_id = config_int("engine.machine_id").unwrap_or(101) as u16;
```

---

## Example AppConfig Structure within business logic

```rust
#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub debug: bool,

    #[serde(default)]
    pub features: HashMap<String, bool>,

    #[serde(default)]
    pub logging: LogConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct LogConfig {
    #[serde(default)]
    pub level: String,

    #[serde(default)]
    pub file: String,
}
```


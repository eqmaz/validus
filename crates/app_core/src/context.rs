use crate::config::{init_global_config, typed_config};
use crate::{console, sout, AppError, Logger};
use ctrlc;
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

// == Type shortcuts -----
type ShutdownHook = Box<dyn FnOnce() + Send + Sync>;
type FeatureFlags = Arc<Mutex<HashMap<String, bool>>>;

// == Feature flag handling ----
/// This is required in user-land to get the feature flags
/// The App config must be a struct that implements this trait
pub trait FeatureMapProvider {
    fn feature_map(&self) -> &HashMap<String, bool>;
}

trait FeatureFlagExt {
    fn is_enabled(&self, key: &str) -> bool;
}

impl FeatureFlagExt for FeatureFlags {
    /// Makes it easier to query feature flag existence later
    fn is_enabled(&self, key: &str) -> bool {
        self.lock().unwrap().get(key).copied().unwrap_or(false)
    }
}

// == App init options -----

/// Options for booting the ConfigManager
/// These are always hard coded from main()
/// Everything else goes in the configs themselves
pub struct AppConfigOptions<T> {
    /// The config struct is defined in user land, contains defaults, and is passed in
    pub config_type: std::marker::PhantomData<T>,
    /// Search paths for the config file. It will search in the order provided.
    pub search_paths: Vec<PathBuf>,
    /// The name of the config file to load - in the future, can support multiple files
    pub file_name: String,
}

/// Options for booting the logger.
/// We need an output stream, and minimum log level - nice and simple
/// For now we just have 1 (thread-safe) logger per application.
/// Any contextual information is added as fields to the log messages
/// This is actually more memory efficient and faster, than having a separate logger instance for each thread
/// Especially because all logs will go to the same destination anyway
/// This can always be changed
#[derive(Debug, Clone)]
pub struct AppLoggerOptions {
    pub log_path: String,
    pub log_level: String, // TODO [later] - probably could use enums for this
}

/// Options for initializing the application.
/// These are passed in to the AppContext init function.
pub struct AppInitOptions<T> {
    pub config: Option<AppConfigOptions<T>>,
    pub logger: Option<AppLoggerOptions>,
    // future: recovery, panic hooks, etc.
}

impl<T> Default for AppInitOptions<T> {
    fn default() -> Self {
        Self {
            config: None,
            logger: None,
        }
    }
}

impl<T> AppInitOptions<T> {
    /// Create new set of initialization options.
    /// Typically followed by calls to `with_config()` and/or `with_logger()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify config options used to initialize the application.
    ///
    /// It just sets up the ability to load, by provisioning the config search paths and target file
    /// It doesn't actually load the config yet. That's done with init is called.
    ///
    /// # Parameters
    /// - `search_paths`: Directories to search for the configuration file.
    /// - `filename`: Name of the config file (e.g., "config.toml").
    ///
    /// # Returns
    /// A modified `AppInitOptions` instance with the configuration settings applied.
    pub fn with_config(mut self, search_paths: Vec<PathBuf>, filename: impl Into<String>) -> Self {
        self.config = Some(AppConfigOptions {
            config_type: std::marker::PhantomData,
            search_paths,
            file_name: filename.into(),
        });
        self
    }

    /// Specify logger settings for app bootstrapping.
    ///
    /// Configures the log output path and the minimum log level.
    /// It does not perform any logging setup immediately — this is done
    /// when init is called on AppContext
    ///
    /// # Parameters
    /// - `path`: File path to write log output to (e.g., "./logs/app.log").
    /// - `level`: Minimum log level (e.g., "info", "debug").
    ///
    /// # Returns
    /// A modified `AppInitOptions` instance with the logger configuration applied.
    pub fn with_logger(mut self, path: impl Into<String>, level: impl Into<String>) -> Self {
        self.logger = Some(AppLoggerOptions {
            log_path: path.into(),
            log_level: level.into(),
        });
        self
    }
}

/// AppContext - Central context that manages lifecycle, shutdown behavior, and runtime features.
///
/// Create using [`AppContext::init`].
///
/// It encapsulates:
/// - Graceful shutdown
/// - Runtime feature flags
/// - Lifecycle hooks
/// - Whatever else, it can be extended
///
/// Intended to be passed throughout the application as needed.
/// It may be extended to support DI features, service registration, etc
pub struct AppContext {
    feature_flags: FeatureFlags,
    shutdown_hooks: Vec<ShutdownHook>,
}

impl AppContext {
    /// Sets up SIGINT / SIGTERM handling
    fn handle_signals(&self) {
        let shutdown_flag = Arc::clone(&self.feature_flags);
        ctrlc::set_handler(move || {
            let msg = "Received termination signal (Ctrl+C or SIGTERM)";
            Logger::warn(msg, None);
            // wout macro only works with string literals
            //wout!(msg.to_string()); // TODO might wanna bring in Logger functionality that will propagate log messages to the console as well
            console::sout(msg);

            let mut flags = shutdown_flag.lock().unwrap();
            flags.insert("terminate_signal".parse().unwrap(), true);
        })
        .expect("Error setting Ctrl-C handler");
    }

    /// Set up the ConfigManager
    fn init_config<T>(opts: &AppInitOptions<T>)
    where
        T: serde::de::DeserializeOwned + Default + Send + Sync + 'static,
    {
        if let Some(cfg) = &opts.config {
            // TODO 1 later - instead of panic, we should return a Result
            // TODO 2 - config manager should keep record of actually loaded config files / sources
            init_global_config::<T>(&cfg.search_paths, &cfg.file_name);
            //ConfigManager::init::<T>(&cfg.search_paths, &cfg.file_name);

            sout!("Config initialized from {}", cfg.file_name);
        }
    }

    /// Set up the logger
    fn init_logger<T>(opts: &AppInitOptions<T>) {
        if let Some(log_opts) = &opts.logger {
            Logger::init(&log_opts.log_path, &log_opts.log_level);
            sout!(
                "Logger initialized to {} [{}]",
                Logger::log_destination().unwrap_or_else(|| "[undefined]".into()),
                log_opts.log_level
            );
        }
    }

    /// Initializes the app context with optional config and logger setup.
    ///
    /// Does the bootstrapping, using the [`AppInitOptions`],
    /// Returns a fully constructed [`AppContext`]
    ///
    /// # Type Parameters
    /// - `T`: The user-defined configuration struct - must be `serde::de::DeserializeOwned`.
    ///        because ConfigManager needs it
    pub fn init<T>(opts: AppInitOptions<T>) -> Self
    where
        T: serde::de::DeserializeOwned + Default + Send + Sync + 'static,
    {
        Self::init_config(&opts);
        Self::init_logger(&opts);
        Self {
            feature_flags: Arc::new(Mutex::new(HashMap::new())),
            shutdown_hooks: vec![],
        }
    }

    /// Inject key-value feature flags
    /// We keep these separate from the config, but they can be gotten from the config anyway
    pub fn with_feature_flags(self, flags: HashMap<String, bool>) -> Self {
        {
            let mut guard = self.feature_flags.lock().unwrap();
            for (k, v) in flags {
                guard.insert(k, v);
            }
        }
        self
    }

    /// Similar to `with_feature_flags`, but loads from the config manager
    /// Looks for the `features` field in the config struct
    pub fn extract_feature_flags<T>(&mut self)
    where
        T: 'static + Send + Sync + FeatureMapProvider + serde::de::DeserializeOwned,
    {
        // Get hold of the config struct,
        // Iterate over the feature flags and save them within our own feature flag map
        let config = typed_config::<T>();
        for (k, v) in config.feature_map() {
            self.feature_flags.lock().unwrap().insert(k.clone(), *v);
        }
    }

    /// Query feature flag at runtime
    pub fn feature_enabled(&self, key: &str) -> bool {
        self.feature_flags.is_enabled(key)
    }

    /// Get a full map of feature flags and their status.
    pub fn feature_flag_map(&self) -> HashMap<String, bool> {
        self.feature_flags.lock().unwrap().clone()
    }

    /// Register shutdown callback(s) (they get executed in reverse order)
    pub fn on_shutdown<F>(&mut self, hook: F)
    where
        // Not using "hook: ShutdownHook", so users can pass in closures without boxing
        F: FnOnce() + Send + Sync + 'static,
    {
        self.shutdown_hooks.push(Box::new(hook));
    }

    // TODO [later] register on_error_created callbacks
    //  to have a global error handler(S) on top of the idiomatic error pipeline
    //  may want to run those callbacks in a separate thread

    /// Run the application entrypoint function (sync - no tokio)
    /// It should return a Result of () or AppError
    pub fn start<F>(mut self, entrypoint: F)
    where
        F: FnOnce(&mut Self) -> Result<(), AppError>,
    {
        self.handle_signals();

        //entrypoint(&mut self);

        if let Err(err) = entrypoint(&mut self) {
            err.log_and_display();
            // std::process::exit(1);
        }
        self.shutdown();
    }

    /// Use start_async to run an async entrypoint (tokio runtime apps, etc)
    /// It's future should return a Result of () or AppError
    pub async fn start_async(
        mut self,
        entrypoint: for<'a> fn(&'a mut Self) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>>,
    ) {
        self.handle_signals();

        let result = entrypoint(&mut self).await;

        if let Err(err) = result {
            err.log_and_display();
        }

        self.shutdown();
    }

    /// Graceful shutdown - calls registered hooks
    pub fn shutdown(&mut self) {
        Logger::info("Shutting down.", None);
        while let Some(hook) = self.shutdown_hooks.pop() {
            hook();
        }
        console::suspend();
        Logger::info("Shutdown complete.", None);
    }
}

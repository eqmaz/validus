//! # app_core
//!
//! Core framework runtime utils for application bootstrapping,
//! config loading, logging, console output, feature flags, and graceful shutdown.
//!
//! ## Usage
//! In the main app, import the prelude for convenient access:
//!
//! ```rust
//! use app_core::prelude::*;
//! ```
//!
//! To enable macros like `sout!`, `wout!`, etc., add:
//! ```rust
//! #[macro_use]
//! extern crate app_core;
//! ```
//!
//! - AppContext lifecycle management
//! - Logger utilities
//! - Console output macros and functions
//! - Feature flags
//! - Error types
//! - Whatever else we build in later
//!
pub mod colors;
pub mod config;
pub mod console;
pub mod context;
pub mod errors;
pub mod logger;

#[macro_use]
pub mod macros;

//pub mod macros;
pub mod prelude;
pub mod utils;

// Re-exports
pub use colors::*;
pub use config::ConfigManager;
pub use console::{colorize, eout, out, resume, set_colors, suspend};
pub use context::{AppConfigOptions, AppContext, AppInitOptions, FeatureMapProvider};
pub use errors::{AppError, ErrorCode};
pub use logger::Logger;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn lib_level_test() {
        assert_eq!(2 + 2, 4);
    }
}

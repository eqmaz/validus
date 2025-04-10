/// Core context + DI
pub use crate::context::{AppContext, AppInitOptions, FeatureMapProvider};

/// Config access (best use via AppContext, not directly where possible)
pub use crate::config::ConfigManager;

/// Logging
pub use crate::logger::Logger;

/// Console output
pub use crate::console::{eout, iout, out, sout, wout};

// About the macros for console - they're in macro.rs
// macros like `sout!`, `wout!`, `out_f!` are available globally
// when doing `use app_core` or declare `#[macro_use] extern crate app_core;`

/// Error types
pub use crate::errors::{AppError, ErrorCode, IntoAppError};

/// Common color constants
pub use crate::colors::{COLOR_BLUE, COLOR_GREEN, COLOR_GREY, COLOR_RED, COLOR_RESET, COLOR_YELLOW};

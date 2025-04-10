// crates/app_core/src/console.rs

#![allow(unused_imports)]
use chrono::Local;
use std::io::{self, IsTerminal}; // Terminal detection for conditional coloring
use std::sync::atomic::{AtomicBool, Ordering}; // Get system timestamps
use std::sync::LazyLock;

// Import ANSI color constants from crate root (they're re-exported via lib.rs)
use crate::{COLOR_BLUE, COLOR_GREEN, COLOR_GREY, COLOR_RED, COLOR_RESET, COLOR_YELLOW};

/// Global flag to suspend console output
/// This uses a Mutex for interior mutability and thread-safety
static SUSPENDED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

/// Whether colours are enabled globally (regardless of TTY)
static COLORS_ENABLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(true));

/// Check once at runtime if stdout is a terminal (TTY),
/// and cache it for future use (avoids per-message checks)
/// This is only set once, so it doesn't need a Mutex
#[cfg(not(test))]
static IS_TTY: LazyLock<bool> = LazyLock::new(|| io::stdout().is_terminal());

#[cfg(test)] // Force on, for unit testing (unit tests are not run in TTY mode)
static IS_TTY: LazyLock<bool> = LazyLock::new(|| true);

/// Returns the current timestamp formatted as `[YYYY-MM-DD HH:MM:SS.mmm]`
fn current_time() -> String {
    Local::now().format("[%Y-%m-%d %H:%M:%S.%3f]").to_string()
}

/// Conditionally colorizes text if the output is a terminal
///
/// # Arguments
/// * `text` - The text to colorize
/// * `color` - ANSI escape code for the color
///
/// # Returns
/// * Colorized string if TTY, otherwise plain string
pub fn colorize(text: &str, color: &str) -> String {
    if *IS_TTY && COLORS_ENABLED.load(Ordering::Relaxed) {
        format!("{}{}{}", color, text, COLOR_RESET)
    } else {
        text.to_string()
    }
}

/// Suspends all console output (except errors via `eout`)
/// Can be useful for suppressing logs during batch tasks or tests
#[allow(dead_code)]
pub fn suspend() {
    SUSPENDED.store(true, Ordering::Relaxed);
}

/// Resumes console output that was suspended
#[allow(dead_code)]
pub fn resume() {
    SUSPENDED.store(false, Ordering::Relaxed);
}

/// Sets terminal colouring on or off. When TTY is false, colors are not applied anyway.
/// So colours are only displayed in the terminal, and when this is true.
pub fn set_colors(enabled: bool) {
    COLORS_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Internal utility to send messages to stdout if output is not suspended
/// Automatically adds a timestamp and grey color to the prefix
fn send(message: String) {
    if !SUSPENDED.load(Ordering::Relaxed) {
        let ts = colorize(&current_time(), COLOR_GREY);
        println!("{} {}", ts, message);
    }
}

/// Public output function, formats a value and sends it with timestamp
///
/// # Arguments
/// * `value` - Any Display-able content
pub fn out<T: std::fmt::Display>(value: T) {
    send(value.to_string());
}

/// Always prints an error to stderr (eprintln),
/// regardless of whether output is suspended
///
/// # Arguments
/// * `code` - A tag or error code label (e.g., "ERROR", "FATAL")
/// * `value` - The error message
#[allow(dead_code)]
pub fn eout<T: std::fmt::Display>(code: &str, value: T) {
    let msg = format!("✖ [{}] {}", code, value.to_string());
    let payload = colorize(&msg, COLOR_RED);
    //eprintln!("{} {:?}", current_time(), payload); // Prints the debug representation (escapes formatting)
    eprintln!("{} {}", colorize(&current_time(), COLOR_GREY), payload);
}

/// Contextual Success output with green checkmark
pub fn sout<T: std::fmt::Display>(value: T) {
    let msg = format!("✔ {}", value.to_string());
    let payload = colorize(&msg, COLOR_GREEN);
    out(&payload);
}

/// Contextual Warning output with yellow exclamation mark
pub fn wout<T: std::fmt::Display>(value: T) {
    let msg = format!("⚠ {}", value.to_string());
    let payload = colorize(&msg, COLOR_YELLOW);
    out(&payload);
}

/// Contextual Info output with blue info mark
pub fn iout<T: std::fmt::Display>(value: T) {
    let msg = format!("ℹ {}", value.to_string());
    let payload = colorize(&msg, COLOR_BLUE);
    out(&payload);
}

// -- the macros live in macros.rs --

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// Unit tests for console.rs
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_suspend_and_resume_output() {
        // Start suspended
        suspend();
        assert!(SUSPENDED.load(Ordering::Relaxed));

        // Resume and check
        resume();
        assert!(!SUSPENDED.load(Ordering::Relaxed));
    }

    #[test]
    fn test_colorize_when_disabled() {
        set_colors(false);
        let result = colorize("Hello", COLOR_GREEN);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_current_time_format() {
        let ts = current_time();
        assert!(ts.starts_with('[') && ts.ends_with(']'));
        assert!(ts.contains(':'));
    }

    #[test]
    fn test_colorize() {
        // We need to force IS_TTY to be true
        let result = colorize("Hello", COLOR_GREEN);
        assert!(result.starts_with(COLOR_GREEN));
        assert!(result.ends_with(COLOR_RESET));
    }
}

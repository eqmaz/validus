/**

fn warn_and_console(msg: &str) {
    Logger::warn(msg, None);
    wout!(msg);
}
TODO -create macros like this
warn_and_console!("SIGTERM received! Shutting down...");

*/

/// Macro: Print a formatted message to stdout with timestamp
/// Use this instead of `println!` to integrate with the console system
#[macro_export]
macro_rules! out_f {
    ($($arg:tt)*) => {
        $crate::console::out(format!($($arg)*));
    };
}

/// Macro: Print a green ✔ success message
/// Usage: `sout!("Saved successfully: {}", id);`
#[macro_export]
macro_rules! sout {
    ($($arg:tt)*) => {
        $crate::console::out(
            $crate::console::colorize(
                &format!("✔ {}", format!($($arg)*)),
                $crate::COLOR_GREEN
            )
        );
    };
}

/// Macro: Print a yellow ⚠ warning message
/// Usage: `wout!("Missing optional field: {}", field);`
#[macro_export]
macro_rules! wout {
    ($($arg:tt)*) => {
        $crate::console::out(
            $crate::console::colorize(
                &format!("⚠ {}", format!($($arg)*)),
                $crate::COLOR_YELLOW
            )
        );
    };
}

/// Macro: Print a blue ℹ info message
/// Usage: `iout!("Retrying connection...");`
#[macro_export]
macro_rules! iout {
    ($($arg:tt)*) => {
        $crate::console::out(
            $crate::console::colorize(
                &format!("ℹ {}", format!($($arg)*)),
                $crate::COLOR_BLUE
            )
        );
    };
}

// #[macro_export]
// macro_rules! warn_and_console {
//     ($msg:expr) => {{
//         $crate::Logger::warn($msg, None);
//         $crate::wout!($msg);
//     }};
// }

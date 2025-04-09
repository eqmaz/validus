use app_core::console::{colorize, iout, resume, set_colors, sout, suspend, wout};

#[test]
fn test_colorized_output_looks_right() {
    use app_core::console::{colorize, set_colors};
    use app_core::{COLOR_GREEN, COLOR_RESET};

    set_colors(true);
    let val = "hello world";
    let out = colorize(val, COLOR_GREEN);
    assert!(out.starts_with(COLOR_GREEN));
    assert!(out.ends_with(COLOR_RESET));
}

#[test]
fn test_console_public_api_usage() {
    suspend();
    resume(); // sanity check that these compile and run

    set_colors(true);
    let colored = colorize("test", "\x1b[32m");
    assert!(colored.contains("\x1b[32m"));

    sout("All good");
    wout("Careful");
    iout("FYI");
}

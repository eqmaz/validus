use std::{fs, io};

/// Get the current memory usage of this process in MB
/// Works only on Linux
pub fn get_memory_usage_mb() -> io::Result<f64> {
    let status = fs::read_to_string("/proc/self/status")?;

    for line in status.lines() {
        if line.starts_with("VmRSS:") {
            // line is like: "VmRSS:\t   123456 kB"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(kb) = parts[1].parse::<f64>() {
                    let mut mb = kb / 1024.0; // Convert kB to MB
                    mb = (mb * 1000.0).round() / 1000.0; // round mb to 3dp
                    return Ok(mb);
                }
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, "VmRSS not found"))
}

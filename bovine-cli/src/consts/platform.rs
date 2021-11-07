#[cfg(not(target_os = "windows"))]
pub const NEWLINE: &str = "\n";
#[cfg(target_os = "windows")]
pub const NEWLINE: &str = "\r\n";

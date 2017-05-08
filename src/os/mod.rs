#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(all(unix, not(target_os = "macos")))]
pub mod unix;

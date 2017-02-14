pub use libc::*;

#[cfg(target_os = "macos")]
pub const AT_FDCWD: c_int = -2;

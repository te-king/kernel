#[cfg(target_arch = "aarch64")]
#[path = "aarch64/mod.rs"]
pub mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
pub mod arch;
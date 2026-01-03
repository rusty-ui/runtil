#[cfg(target_os = "macos")]
pub mod appkit;

#[cfg(target_os = "macos")]
pub use appkit::implementation::AppkitEventPump as EventPumpImpl;

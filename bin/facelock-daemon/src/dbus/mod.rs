//! DBus Interface for FaceLock
//!
//! This module will handle all the properties, methods, and signals for the FaceLock DBus interface.

/// This struct represents the FaceLock DBus interface.
///
/// Contains all the states and fields required to run the DBus interface.
pub struct FaceLockDBusInterface {
    version: String,
}

impl FaceLockDBusInterface {
    pub fn new() -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();
        Self { version }
    }
}

#[zbus::interface(name = "org.rde.FaceLock")]
impl FaceLockDBusInterface {
    /// Returns the version of the FaceLock service.
    #[zbus(property)]
    pub fn version(&self) -> &str {
        &self.version
    }
}

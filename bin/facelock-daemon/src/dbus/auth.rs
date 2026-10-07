//! DBus interface for Authentication
//!
//! This module will handle all the properties, methods, and signals for
//!    - the authentication of users using their face data

/// This struct represents the FaceLock Authentication DBus interface.
pub struct FaceLockAuthDBusInterface {
    // FIXME: Add fields for the Face Authentication DBus interface
}

// TODO: Implement the DBus interface for Face Authentication
impl FaceLockAuthDBusInterface {
    pub fn new() -> Self {
        Self {}
    }
}

// TODO: Implement the DBus interface for Face Authentication
#[zbus::interface(name = "org.rde.FaceLock.Auth")]
impl FaceLockAuthDBusInterface {}

//! DBus Interface for Face Data Management.
//!
//! This module will handle all the properties, methods, and signals for
//!    - the deletion of users and their associated face data
//!    - giving different access levels to different users

/// This struct represents the FaceLock Authentication DBus interface.
pub struct FaceLockManagerDBusInterface {
    // FIXME: Add fields for the Face Management DBus interface
}

// TODO: Implement the DBus interface for Face Management
impl FaceLockManagerDBusInterface {
    pub fn new() -> Self {
        Self {}
    }
}

// TODO: Implement the DBus interface for Face Management
#[zbus::interface(name = "org.rde.FaceLock.Manager")]
impl FaceLockManagerDBusInterface {}

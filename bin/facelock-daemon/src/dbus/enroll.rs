//! DBus Interface for new Face Enrollment.
//!
//! This module will handle all the properties, methods, and signals for
//!     - the enrollment of new users and their associated face data
//!     - updating existing user face data

/// This struct represents the FaceLock Enrollment DBus interface.
pub struct FaceLockEnrollDBusInterface {
    // FIXME: Add fields for the Face Enrollment DBus interface
}

// TODO: Implement the DBus interface for Face Enrollment
impl FaceLockEnrollDBusInterface {
    pub fn new() -> Self {
        Self {}
    }
}

// TODO: Implement the DBus interface for Face Enrollment
#[zbus::interface(name = "org.rde.FaceLock.Enroll")]
impl FaceLockEnrollDBusInterface {}

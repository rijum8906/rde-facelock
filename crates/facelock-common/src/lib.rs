//! Shared types for facelock workspace
//!
//! This crate is the bottom of the dependency graph and must not include any other facelock crates
//!
//! - [`error`]  — the workspace-wide [`error::FaceLockError`] and [`error::Result`] alias
//!
//! # Design Constraints
//!
//! - No `unsafe`. Enforced by `#![forbid(unsafe_code)]`.
//! - No I/O beyond config loading and path construction.
//! - No dependency on `tokio`, `zbus`, `ort`, `nokhwa`, or any other
//!   heavy crate. Only `serde`, `toml`, `thiserror`, and `tracing`.
//! - Public types are `#[non_exhaustive]` where they may grow.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2024_compatibility)]
#![warn(unreachable_pub)]
#![warn(clippy::all)]

pub mod error;

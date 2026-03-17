//! Business logic layer for the InkTix contract.
//!
//! Organizes domain logic into core (always available), sports (feature-gated),
//! and concert (feature-gated) submodules that operate on `InkTixStorage`.

pub mod core;

#[cfg(feature = "sports")]
pub mod sports;

#[cfg(feature = "concert")]
pub mod concert;

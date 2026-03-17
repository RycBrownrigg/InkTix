//! Test modules for the InkTix contract.
//!
//! Organizes test helpers into core, sports (feature-gated), and concert
//! (feature-gated) submodules. Primary tests live inline in `lib.rs`.

pub mod core_tests;

#[cfg(feature = "sports")]
pub mod sports_tests;

#[cfg(feature = "concert")]
pub mod concert_tests;

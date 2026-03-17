//! Top-level type definitions for the InkTix contract.
//!
//! Re-exports all domain types from the core, sports, and concert submodules
//! so that consumers can import via `crate::types::*`.

pub mod core;
pub mod sports;
pub mod concert;

pub use core::*;
pub use sports::*;
pub use concert::*;

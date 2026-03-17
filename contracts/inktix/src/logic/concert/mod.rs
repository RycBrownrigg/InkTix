//! Concert-specific business logic modules.
//!
//! Re-exports artist management logic gated behind the "concert" feature flag.

pub mod artist_management;

pub use artist_management::*;

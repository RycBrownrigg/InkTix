//! Storage layer for the InkTix contract.
//!
//! Provides the primary `InkTixStorage` struct (ink! storage item) and
//! a standalone `AnalyticsStorage` helper for analytics initialization.

pub mod contract_storage;
pub mod analytics_storage;

pub use contract_storage::*;
pub use analytics_storage::*;

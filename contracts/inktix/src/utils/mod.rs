//! Utility functions for validation and currency conversion.
//!
//! Provides reusable helpers for input validation (non-empty strings,
//! positive numbers) and DOT-equivalent currency conversions.

pub mod validation;
pub mod conversion;

pub use validation::*;
pub use conversion::*;

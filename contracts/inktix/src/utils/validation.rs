//! Input validation helpers.
//!
//! Provides guard functions that return `InkTixError::InvalidData` for
//! empty strings, zero-value numbers, and zero-balance amounts.
//!
//! # Functions
//! - `validate_non_empty_string` -- rejects empty or whitespace-only strings
//! - `validate_positive_number` -- rejects zero-valued u32 inputs
//! - `validate_positive_balance` -- rejects zero-valued u128 inputs

use crate::types::core::error::{InkTixError, InkTixResult};

/// Validate that a string is not empty
pub fn validate_non_empty_string(value: &str, _field_name: &str) -> InkTixResult<()> {
    if value.trim().is_empty() {
        return Err(InkTixError::InvalidData);
    }
    Ok(())
}

/// Validate that a number is greater than zero
pub fn validate_positive_number(value: u32, _field_name: &str) -> InkTixResult<()> {
    if value == 0 {
        return Err(InkTixError::InvalidData);
    }
    Ok(())
}

/// Validate that a balance is greater than zero
pub fn validate_positive_balance(value: u128, _field_name: &str) -> InkTixResult<()> {
    if value == 0 {
        return Err(InkTixError::InvalidData);
    }
    Ok(())
}

use crate::types::{InkTixError, InkTixResult};

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
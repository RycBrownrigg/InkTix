use crate::types::{InkTixError, InkTixResult};

/// Validation utility functions
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validate that a string is not empty
    pub fn validate_non_empty_string(value: &str, field_name: &str) -> InkTixResult<()> {
        if value.trim().is_empty() {
            return Err(InkTixError::InvalidData);
        }
        Ok(())
    }

    /// Validate that a number is greater than zero
    pub fn validate_positive_number(value: u32, field_name: &str) -> InkTixResult<()> {
        if value == 0 {
            return Err(InkTixError::InvalidData);
        }
        Ok(())
    }

    /// Validate that a balance is greater than zero
    pub fn validate_positive_balance(value: u128, field_name: &str) -> InkTixResult<()> {
        if value == 0 {
            return Err(InkTixError::InvalidData);
        }
        Ok(())
    }

    /// Validate event timing
    pub fn validate_event_timing(
        date: u64,
        doors_open: u64,
        show_start: u64,
        estimated_end: u64,
    ) -> InkTixResult<()> {
        if doors_open >= show_start || show_start >= estimated_end || date > doors_open {
            return Err(InkTixError::InvalidData);
        }
        Ok(())
    }

    /// Validate supporting artists count
    pub fn validate_supporting_artists_count(count: usize) -> InkTixResult<()> {
        if count > 10 {
            return Err(InkTixError::InvalidData);
        }
        Ok(())
    }

    /// Validate discount percentage
    pub fn validate_discount_percentage(percentage: u8) -> InkTixResult<()> {
        if percentage > 50 {
            return Err(InkTixError::InvalidData);
        }
        Ok(())
    }
}
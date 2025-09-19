use crate::types::{CurrencyId, InkTixError, InkTixResult};

/// Convert amount to DOT equivalent
pub fn convert_to_dot_equivalent(
    amount: u128,
    currency: CurrencyId,
    rates: &[u128],
) -> InkTixResult<u128> {
    match currency {
        CurrencyId::DOT => Ok(amount),
        _ => {
            let rate = rates[currency as usize];
            if rate == 0 {
                return Err(InkTixError::InvalidCurrency);
            }
            let dot_amount = amount.saturating_mul(rate) / 1_000_000_000_000;
            if dot_amount == 0 && amount > 0 {
                return Err(InkTixError::CurrencyConversionFailed);
            }
            Ok(dot_amount)
        }
    }
}

/// Convert from DOT equivalent to target currency
pub fn convert_from_dot_equivalent(
    dot_amount: u128,
    target_currency: CurrencyId,
    rates: &[u128],
) -> InkTixResult<u128> {
    match target_currency {
        CurrencyId::DOT => Ok(dot_amount),
        _ => {
            let rate = rates[target_currency as usize];
            if rate == 0 {
                return Err(InkTixError::InvalidCurrency);
            }
            #[allow(clippy::arithmetic_side_effects)]
            let target_amount = dot_amount.saturating_mul(1_000_000_000_000) / rate;
            Ok(target_amount)
        }
    }
}

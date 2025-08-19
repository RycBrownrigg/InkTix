use crate::types::{CurrencyId, InkTixError, InkTixResult};
use ink::storage::Mapping;

/// Currency management business logic
pub struct CurrencyManager {
    pub supported_currencies: Vec<CurrencyId>,
    pub currency_rates: Mapping<CurrencyId, u128>,
}

impl CurrencyManager {
    pub fn new() -> Self {
        let mut supported_currencies = Vec::new();
        supported_currencies.push(CurrencyId::DOT);
        supported_currencies.push(CurrencyId::ACA);
        supported_currencies.push(CurrencyId::AUSD);
        supported_currencies.push(CurrencyId::LDOT);
        supported_currencies.push(CurrencyId::KSM);

        let mut manager = Self {
            supported_currencies,
            currency_rates: Mapping::new(),
        };

        // Initialize currency rates (DOT as base)
        manager.currency_rates.insert(CurrencyId::DOT, &1_000_000_000_000);
        manager.currency_rates.insert(CurrencyId::ACA, &50_000_000_000);
        manager.currency_rates.insert(CurrencyId::AUSD, &150_000_000_000);
        manager.currency_rates.insert(CurrencyId::LDOT, &950_000_000_000);
        manager.currency_rates.insert(CurrencyId::KSM, &15_000_000_000_000);

        manager
    }

    /// Update currency exchange rate
    pub fn update_currency_rate(&mut self, currency: CurrencyId, rate_to_dot: u128) -> InkTixResult<()> {
        if rate_to_dot == 0 {
            return Err(InkTixError::InvalidData);
        }

        self.currency_rates.insert(currency, &rate_to_dot);
        Ok(())
    }

    /// Convert amount to DOT equivalent
    pub fn convert_to_dot_equivalent(&self, amount: u128, currency: CurrencyId) -> InkTixResult<u128> {
        match currency {
            CurrencyId::DOT => Ok(amount),
            _ => {
                let rate = self.currency_rates.get(currency).ok_or(InkTixError::InvalidData)?;
                let dot_amount = amount.saturating_mul(rate) / 1_000_000_000_000;
                if dot_amount == 0 && amount > 0 {
                    return Err(InkTixError::InvalidData);
                }
                Ok(dot_amount)
            }
        }
    }

    /// Convert from DOT equivalent to target currency
    pub fn convert_from_dot_equivalent(&self, dot_amount: u128, target_currency: CurrencyId) -> InkTixResult<u128> {
        match target_currency {
            CurrencyId::DOT => Ok(dot_amount),
            _ => {
                let rate = self.currency_rates.get(target_currency).ok_or(InkTixError::InvalidData)?;
                if rate == 0 {
                    return Err(InkTixError::InvalidData);
                }
                let target_amount = dot_amount.saturating_mul(1_000_000_000_000) / rate;
                Ok(target_amount)
            }
        }
    }

    /// Get supported currencies
    pub fn get_supported_currencies(&self) -> Vec<CurrencyId> {
        self.supported_currencies.clone()
    }

    /// Get currency exchange rate
    pub fn get_currency_rate(&self, currency: CurrencyId) -> Option<u128> {
        self.currency_rates.get(currency)
    }
}
use crate::storage::*;
use crate::types::*;
use ink::prelude::string::String;
use ink::prelude::string::ToString;

/// Currency management functionality
pub struct CurrencyManagement;

impl CurrencyManagement {
    /// Update currency rate
    pub fn update_currency_rate(
        storage: &mut SportsBrokerStorage,
        currency: CurrencyId,
        rate: u128,
    ) -> Result<(), String> {
        if rate == 0 {
            return Err("Currency rate cannot be zero".to_string());
        }
        
        storage.currency_rates.insert(currency, &rate);
        Ok(())
    }
}
use ink::primitives::AccountId;
use crate::storage::contract_storage::SportsBrokerStorage;
use crate::types::*;

/// Currency management logic
pub struct CurrencyManagement;

impl CurrencyManagement {
    /// Add a new supported currency
    pub fn add_supported_currency(
        storage: &mut SportsBrokerStorage,
        currency: CurrencyId,
        initial_rate: u128,
    ) -> Result<(), String> {
        if storage.supported_currencies.contains(&currency) {
            return Err("Currency already supported".to_string());
        }
        
        storage.supported_currencies.push(currency);
        storage.currency_rates.insert(currency, &initial_rate);
        
        Ok(())
    }
    
    /// Update exchange rate for a currency
    pub fn update_currency_rate(
        storage: &mut SportsBrokerStorage,
        currency: CurrencyId,
        new_rate: u128,
    ) -> Result<(), String> {
        if !storage.supported_currencies.contains(&currency) {
            return Err("Currency not supported".to_string());
        }
        
        storage.currency_rates.insert(currency, &new_rate);
        
        Ok(())
    }
    
    /// Remove a supported currency
    pub fn remove_supported_currency(
        storage: &mut SportsBrokerStorage,
        currency: CurrencyId,
    ) -> Result<(), String> {
        if !storage.supported_currencies.contains(&currency) {
            return Err("Currency not supported".to_string());
        }
        
        // Check if currency is in use
        if Self::is_currency_in_use(storage, currency) {
            return Err("Cannot remove currency that is in use".to_string());
        }
        
        // Remove from supported currencies
        storage.supported_currencies.retain(|&c| c != currency);
        
        // Remove rate
        storage.currency_rates.remove(currency);
        
        Ok(())
    }
    
    /// Convert amount from one currency to another
    pub fn convert_currency(
        storage: &SportsBrokerStorage,
        amount: u128,
        from_currency: CurrencyId,
        to_currency: CurrencyId,
    ) -> Result<u128, String> {
        if from_currency == to_currency {
            return Ok(amount);
        }
        
        let from_rate = storage.currency_rates.get(from_currency)
            .ok_or("Source currency rate not found")?;
        let to_rate = storage.currency_rates.get(to_currency)
            .ok_or("Target currency rate not found")?;
        
        // Convert through DOT as base currency
        let dot_amount = amount * from_rate;
        let converted_amount = dot_amount / to_rate;
        
        Ok(converted_amount)
    }
    
    /// Get current exchange rate for a currency
    pub fn get_currency_rate(
        storage: &SportsBrokerStorage,
        currency: CurrencyId,
    ) -> Option<u128> {
        storage.currency_rates.get(currency)
    }
    
    /// Get list of supported currencies
    pub fn get_supported_currencies(storage: &SportsBrokerStorage) -> Vec<CurrencyId> {
        storage.supported_currencies.clone()
    }
    
    /// Calculate ticket price in a specific currency
    pub fn calculate_ticket_price_in_currency(
        storage: &SportsBrokerStorage,
        base_price_dot: u128,
        target_currency: CurrencyId,
    ) -> Result<u128, String> {
        let target_rate = storage.currency_rates.get(target_currency)
            .ok_or("Target currency rate not found")?;
        
        let price_in_currency = base_price_dot / target_rate;
        Ok(price_in_currency)
    }
    
    /// Get currency information
    pub fn get_currency_info(currency: CurrencyId) -> CurrencyInfo {
        match currency {
            CurrencyId::DOT => CurrencyInfo {
                symbol: "DOT".to_string(),
                name: "Polkadot".to_string(),
                decimals: 10,
                is_stable: false,
            },
            CurrencyId::ACA => CurrencyInfo {
                symbol: "ACA".to_string(),
                name: "Acala".to_string(),
                decimals: 12,
                is_stable: false,
            },
            CurrencyId::AUSD => CurrencyInfo {
                symbol: "aUSD".to_string(),
                name: "Acala Dollar".to_string(),
                decimals: 12,
                is_stable: true,
            },
            CurrencyId::LDOT => CurrencyInfo {
                symbol: "LDOT".to_string(),
                name: "Liquid DOT".to_string(),
                decimals: 10,
                is_stable: false,
            },
            CurrencyId::KSM => CurrencyInfo {
                symbol: "KSM".to_string(),
                name: "Kusama".to_string(),
                decimals: 12,
                is_stable: false,
            },
        }
    }
    
    /// Validate currency amount
    pub fn validate_currency_amount(
        storage: &SportsBrokerStorage,
        currency: CurrencyId,
        amount: u128,
    ) -> Result<bool, String> {
        if !storage.supported_currencies.contains(&currency) {
            return Err("Currency not supported".to_string());
        }
        
        let max_amount = Self::get_max_amount_for_currency(currency);
        if amount > max_amount {
            return Err("Amount exceeds maximum allowed".to_string());
        }
        
        Ok(true)
    }
    
    /// Get maximum amount allowed for a currency
    pub fn get_max_amount_for_currency(currency: CurrencyId) -> u128 {
        match currency {
            CurrencyId::DOT => 1_000_000_000_000, // 1 trillion
            CurrencyId::ACA => 1_000_000_000_000, // 1 trillion
            CurrencyId::AUSD => 1_000_000_000_000, // 1 trillion
            CurrencyId::LDOT => 1_000_000_000_000, // 1 trillion
            CurrencyId::KSM => 1_000_000_000_000, // 1 trillion
        }
    }
    
    /// Check if a currency is currently in use
    pub fn is_currency_in_use(
        storage: &SportsBrokerStorage,
        currency: CurrencyId,
    ) -> bool {
        // Check if any tickets use this currency
        for ticket_id in 1..=storage.total_tickets {
            if let Some(ticket) = storage.tickets.get(ticket_id) {
                if ticket.purchase_currency == currency {
                    return true;
                }
            }
        }
        
        // Check if any revenue is recorded in this currency
        if let Some(revenue) = storage.currency_revenue.get(currency) {
            if revenue > 0 {
                return true;
            }
        }
        
        false
    }
    
    /// Get currency conversion history
    pub fn get_currency_conversion_history(
        _storage: &SportsBrokerStorage,
        _currency: CurrencyId,
    ) -> Vec<(u64, u128, u128)> { // timestamp, amount, rate
        // Placeholder implementation
        Vec::new()
    }
    
    /// Record a currency conversion
    pub fn record_currency_conversion(
        _storage: &mut SportsBrokerStorage,
        _from_currency: CurrencyId,
        _to_currency: CurrencyId,
        _amount: u128,
        _rate: u128,
    ) {
        // Placeholder implementation
    }
    
    /// Get currency statistics
    pub fn get_currency_statistics(
        storage: &SportsBrokerStorage,
        currency: CurrencyId,
    ) -> CurrencyStatistics {
        let total_revenue = storage.currency_revenue.get(currency).unwrap_or(0);
        let total_conversion_volume = 0; // Not tracked in current implementation
        let total_conversions = 0; // Not tracked in current implementation
        
        CurrencyStatistics {
            currency,
            total_revenue,
            total_conversion_volume,
            total_conversions,
            last_updated: 0, // Will be set by caller
        }
    }
}

/// Currency information structure
#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyInfo {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub is_stable: bool,
}

/// Currency statistics
#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyStatistics {
    pub currency: CurrencyId,
    pub total_revenue: u128,
    pub total_conversion_volume: u128,
    pub total_conversions: u32,
    pub last_updated: u64,
}

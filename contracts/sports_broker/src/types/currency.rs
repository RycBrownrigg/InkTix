/// Multi-currency support for Acala integration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum CurrencyId {
    DOT = 1,
    ACA = 2,
    AUSD = 3,
    LDOT = 4,
    KSM = 5,
}

/// Currency conversion rates and management
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct CurrencyRates {
    pub currency: CurrencyId,
    pub rate_to_dot: u128, // Rate in smallest unit (e.g., planck for DOT)
    pub last_updated: u64,
    pub is_active: bool,
}

/// Currency conversion trait for flexible implementation
pub trait CurrencyConverter {
    fn convert_to_dot(&self, amount: u128, from_currency: CurrencyId) -> Result<u128, String>;
    fn convert_from_dot(&self, dot_amount: u128, to_currency: CurrencyId) -> Result<u128, String>;
    fn get_exchange_rate(&self, currency: CurrencyId) -> Result<u128, String>;
}
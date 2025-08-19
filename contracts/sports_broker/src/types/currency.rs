use ink::prelude::string::String;

/// Multi-currency support for Acala integration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum CurrencyId {
    DOT,
    ACA,
    AUSD,
    LDOT,
    KSM,
}

/// Currency exchange rates storage
pub struct CurrencyRates {
    pub rates: Vec<u128>,
    pub last_updated: u64,
}

/// Currency conversion trait
pub trait CurrencyConverter {
    fn convert_to_dot_equivalent(&self, amount: u128, currency: CurrencyId) -> Result<u128, String>;
    fn convert_from_dot_equivalent(&self, dot_amount: u128, target_currency: CurrencyId) -> Result<u128, String>;
}
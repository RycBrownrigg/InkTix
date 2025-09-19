use ink::prelude::vec::Vec;

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
    pub rates: Vec<u128>, // Use u128 instead of Balance
    pub supported_currencies: Vec<CurrencyId>,
}

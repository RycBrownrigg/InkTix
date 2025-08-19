use ink::prelude::string::String;

/// Common access levels for tickets
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum AccessLevel {
    Standard,
    Premium,
    VIP,
    AllAccess,
}

/// Common error types
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum InkTixError {
    NotOwner,
    InsufficientPayment,
    InvalidData,
    NotFound,
    IdOverflow,
    InvalidCurrency,
    CurrencyConversionFailed,
}

/// Common result type
pub type InkTixResult<T> = Result<T, InkTixError>;

/// Basic event structure that can be extended
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct BaseEvent {
    pub id: u32,
    pub name: String,
    pub venue: String,
    pub date: u64,
    pub capacity: u32,
    pub sold_tickets: u32,
    pub base_price: u128, // Use u128 instead of Balance
    pub active: bool,
}

/// Basic ticket structure that can be extended
#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct BaseTicket {
    pub id: u64,
    pub event_id: u32,
    pub owner: ink::primitives::AccountId, // Use proper AccountId type
    pub purchase_price: u128, // Use u128 instead of Balance
    pub purchase_date: u64,
    pub seat_number: u32,
    pub transferable: bool,
}
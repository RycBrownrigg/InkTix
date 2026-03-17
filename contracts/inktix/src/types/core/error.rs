//! Error types for the InkTix contract.
//!
//! Provides a unified error enum and a convenience result type alias used
//! across utility and validation functions.

/// Unified error type combining InkTixError and String-based errors
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
    EventNotFound,
    TeamNotFound,
    VenueNotFound,
    TicketNotFound,
    SeasonNotFound,
    ArtistNotFound,
    NotActive,
    SoldOut,
    PurchaseLimitReached,
    NotTicketOwner,
    NotTransferable,
    Custom(ink::prelude::string::String),
}

/// Common result type
pub type InkTixResult<T> = Result<T, InkTixError>;

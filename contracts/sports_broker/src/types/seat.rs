use ink::prelude::string::String;

/// Seat types for sports venues
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SeatType {
    GeneralAdmission,
    Reserved,
    PremiumReserved,
    Club,
    Suite,
    FieldLevel,
    Courtside,
    StudentSection,
}

/// Access levels for different ticket types
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum AccessLevel {
    Standard,
    Premium,
    VIP,
    AllAccess,
}

/// Seat information for ticket purchasing
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Seat {
    pub section: String,
    pub row: String,
    pub seat_number: String,
    pub seat_type: SeatType,
    pub access_level: AccessLevel,
    pub price_multiplier: u32, // 10000 = 1.0x
}

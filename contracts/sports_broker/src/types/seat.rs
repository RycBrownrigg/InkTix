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
use ink::prelude::string::String;

/// Seat information structure
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct SeatInfo {
    pub section: String,
    pub row: String,
    pub seat_number: u32,
    pub seat_type: SeatType,
}

/// Seat type enumeration
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum SeatType {
    GeneralAdmission,
    Reserved,
    Premium,
    VIP,
    AllAccess,
}
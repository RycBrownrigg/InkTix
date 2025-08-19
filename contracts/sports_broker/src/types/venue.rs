use ink::prelude::string::String;

/// Enhanced Venue structure
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Venue {
    pub id: u32,
    pub name: String,
    pub city: String,
    pub capacity: u32,
}
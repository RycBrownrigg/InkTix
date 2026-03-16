use ink::prelude::string::String;
use ink::primitives::AccountId;

/// Artist struct from concert_broker
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Artist {
    pub id: u32,
    pub name: String,
    pub verified: bool,
    pub account: Option<AccountId>,
}

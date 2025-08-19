use ink::prelude::string::String;
use crate::types::SportType;

/// Enhanced Season structure
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Season {
    pub id: u32,
    pub name: String,
    pub sport_type: SportType,
    pub start_date: u64,
    pub end_date: u64,
    pub regular_season_games: u32,
    pub active: bool,
    pub season_pass_base_price: u128,
    pub early_bird_discount: u8,
    pub early_bird_deadline: u64,
}
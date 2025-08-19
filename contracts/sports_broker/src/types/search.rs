use crate::types::{SportType, GameType};

/// Search filters for advanced event discovery
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct EventSearchFilter {
    pub sport_type: Option<SportType>,
    pub team_id: Option<u32>,
    pub venue_id: Option<u32>,
    pub min_date: Option<u64>,
    pub max_date: Option<u64>,
    pub game_type: Option<GameType>,
    pub max_price: Option<u128>,
    pub min_availability: Option<u32>, // Minimum available tickets
    pub active_only: bool,
}
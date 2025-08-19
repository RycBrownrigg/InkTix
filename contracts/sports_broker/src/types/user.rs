use ink::prelude::vec::Vec;
use ink::prelude::string::String;
use crate::types::CurrencyId;

/// Loyalty tiers for user management
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum LoyaltyTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

/// User profile for fan management
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct UserProfile {
    pub verified_fan: bool,
    pub favorite_teams: Vec<u32>,
    pub home_city: String,
    pub loyalty_tier: LoyaltyTier,
    pub total_games_attended: u32,
    pub account_creation_date: u64,
    pub anti_scalping_verified: bool,
    pub social_media_verified: bool,
    pub season_pass_holder: bool,
    pub preferred_currency: CurrencyId,
    pub total_spent: u128,
}